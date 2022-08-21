use itertools::Itertools;

use crate::model::{Order, Train};
use crate::network::Network;
use crate::output::Move;
use crate::solvers::utils::{find_nearest_train, group_orders_by_location};
use crate::Output;

use super::utils::calculate_best_route_for_distribution;

pub fn deliver_order_by_nearest_train_multiload_distribution(
    network: &Network,
    mut trains: Vec<Train>,
    mut orders: Vec<Order>,
) -> anyhow::Result<Output> {
    let max_train_capacity = trains.iter().map(|t| t.capacity()).max().unwrap();

    let mut moves = Vec::new();

    orders.retain(|o| !o.is_delivered());

    while !orders.is_empty() {
        log::debug!("NEED TO DELIVER {orders:?}");

        let delivered = group_orders_by_location(&orders)
            .iter()
            .filter(|(_, w, _)| w <= &max_train_capacity)
            .flat_map(|(location, w, order_refs)| {
                if let Some((train_index, available_at)) =
                    find_nearest_train(network, &trains, location, *w)
                {
                    // Remove train from the idle pool.
                    let mut train = trains.remove(train_index);

                    log::debug!(
                        "TRAIN {} from {} to {}, departure={}, arrival={}",
                        train.name(),
                        train.location().name(),
                        location.name(),
                        train.traveled_time(),
                        available_at
                    );

                    if train.location() != location {
                        // Move train to location.
                        moves.push(Move::new(
                            train.traveled_time(),
                            train.name().to_owned(),
                            train.location().name().to_owned(),
                            vec![],
                            location.name().to_owned(),
                            vec![],
                        ));

                        log::debug!("{:?}", moves.last().unwrap());

                        train.move_to(location, network.distance(train.location(), location));
                    }

                    let destinations = order_refs.iter().map(|o| o.destination()).collect_vec();
                    let (route, _) =
                        calculate_best_route_for_distribution(network, location, &destinations);

                    log::debug!("{location} load orders {order_refs:?}");
                    let mut delivery = order_refs.clone();

                    for station in &route {
                        order_refs
                            .iter()
                            .filter(|o| &o.destination() == station)
                            .for_each(|drop_order| {
                                let order_names =
                                    delivery.iter().map(|o| o.name().to_owned()).collect_vec();

                                log::debug!(
                                    "TRAIN {} from={} load={:?} to={} unload={:?} distance={}",
                                    train.name(),
                                    train.location().name(),
                                    order_names,
                                    station.name(),
                                    order_names,
                                    network.distance(train.location(), station)
                                );
                                moves.push(Move::new(
                                    train.traveled_time(),
                                    train.name().to_owned(),
                                    train.location().name().to_owned(),
                                    order_names.clone(),
                                    station.name().to_owned(),
                                    order_names,
                                ));
                                log::debug!("{:?}", moves.last().unwrap());

                                delivery.retain(|o| o != drop_order);
                                train.move_to(station, network.distance(train.location(), station));
                            });
                    }
                    assert!(delivery.is_empty(), "Undelivered orders!: {delivery:?}");

                    // Return train to the idle pool with updated time.
                    trains.push(train);
                } else {
                    log::error!("No train available for {location} with capacity {w}",);
                }
                order_refs.iter().map(|o| o.name().to_owned()).collect_vec()
            })
            .collect_vec();

        if !delivered.is_empty() {
            log::debug!("Orders {delivered:?} delivered");
            delivered.into_iter().for_each(|order| {
                orders.retain(|o| o.name() != order);
            });
        } else {
            log::debug!("No orders delivered");
            break;
        }
    }
    Ok(Output::new(
        moves,
        trains.iter().map(|t| t.traveled_time()).max().unwrap(),
    ))
}
