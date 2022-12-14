use anyhow::bail;
use itertools::Itertools;

use crate::model::{Order, Train};
use crate::output::Move;
use crate::solver::utils::{
    calculate_best_route_for_distribution, find_nearest_train, group_orders_by_location,
};
use crate::Solution;

use super::Algorithm;

#[derive(Debug)]
pub struct NearestTrainOrderDistributionAlgorithm;

impl Algorithm for NearestTrainOrderDistributionAlgorithm {
    fn solve(
        &self,
        orders: Vec<Order>,
        trains: Vec<Train>,
        distance: &dyn Fn(&crate::model::Station, &crate::model::Station) -> u32,
    ) -> anyhow::Result<Solution> {
        let mut orders = orders;
        let mut trains = trains;

        let mut moves = Vec::new();

        orders.retain(|o| !o.is_delivered());

        while !orders.is_empty() {
            log::debug!("NEED TO DELIVER {orders:?}");

            let delivered = group_orders_by_location(&orders)
                .iter()
                .flat_map(|(location, w, order_refs)| {
                    // Try to deliver as many items as possible.
                    let mut orders_to_deliver = order_refs.clone();
                    orders_to_deliver.sort_by_key(|o| o.weight());
                    orders_to_deliver.reverse();

                    let mut nearest_train = None;

                    while nearest_train.is_none() && !orders_to_deliver.is_empty() {
                        let total_weight =
                            orders_to_deliver.iter().map(|o| o.weight()).sum::<u32>();

                        nearest_train =
                            find_nearest_train(&distance, &trains, location, total_weight);

                        if nearest_train.is_none() {
                            orders_to_deliver.pop();
                        }
                    }

                    if let Some((train_index, available_at)) = nearest_train {
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

                            train.move_to(location, distance(train.location(), location));
                        }

                        let destinations = orders_to_deliver
                            .iter()
                            .map(|o| o.destination())
                            .collect_vec();
                        let (route, _) = calculate_best_route_for_distribution(
                            &distance,
                            location,
                            &destinations,
                        );

                        log::debug!("{location} load orders {order_refs:?}");
                        let mut delivery = orders_to_deliver.clone();

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
                                        distance(train.location(), station)
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
                                    train.move_to(station, distance(train.location(), station));
                                });
                        }
                        assert!(delivery.is_empty(), "Undelivered orders!: {delivery:?}");

                        // Return train to the idle pool with updated time.
                        trains.push(train);
                    } else {
                        log::error!("No train available for {location} with capacity {w}",);
                    }
                    orders_to_deliver
                        .iter()
                        .map(|o| o.name().to_owned())
                        .collect_vec()
                })
                .collect_vec();

            if !delivered.is_empty() {
                log::debug!("Orders {delivered:?} delivered");
                delivered.into_iter().for_each(|order| {
                    orders.retain(|o| o.name() != order);
                });
            } else {
                bail!(
                    "Failed to deliver orders: {}",
                    orders
                        .iter()
                        .map(|o| format!(
                            "{}[{}] {}=>{},",
                            o.name(),
                            o.weight(),
                            o.location(),
                            o.destination()
                        ))
                        .collect::<String>()
                );
            }
        }
        Ok(Solution::new(
            moves,
            trains.iter().map(|t| t.traveled_time()).max().unwrap(),
        ))
    }

    fn sort_sensitive(&self) -> bool {
        false
    }
}
