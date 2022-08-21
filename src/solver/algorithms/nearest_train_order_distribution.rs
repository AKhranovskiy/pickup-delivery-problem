use itertools::Itertools;

use crate::model::{Order, Train};
use crate::output::Move;
use crate::solver::utils::{
    calculate_best_route_for_collection, find_nearest_train, group_orders_by_destination,
};
use crate::solver::Algorithm;
use crate::Solution;

#[derive(Debug)]
pub struct NearestTrainOrderDistributionAlgorithm;

impl Algorithm for NearestTrainOrderDistributionAlgorithm {
    fn solve(
        &self,
        orders: Vec<Order>,
        trains: Vec<Train>,
        distance: &dyn Fn(&crate::model::Station, &crate::model::Station) -> u32,
    ) -> anyhow::Result<Solution> {
        let mut trains = trains;
        let mut orders = orders;

        let max_train_capacity = trains.iter().map(|t| t.capacity()).max().unwrap();

        let mut moves = Vec::new();

        orders.retain(|o| !o.is_delivered());

        while !orders.is_empty() {
            log::debug!("NEED TO DELIVER {orders:?}");

            let delivered = group_orders_by_destination(&orders)
                .iter()
                .filter(|(_, w, _)| w <= &max_train_capacity)
                .flat_map(|(destination, w, order_refs)| {
                    // Find the nearest train to the beginning of the route.
                    let location = order_refs.first().unwrap().location();

                    if let Some((train_index, available_at)) =
                        find_nearest_train(&distance, &trains, &location, *w)
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

                        if train.location() != &location {
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

                            train.move_to(&location, distance(train.location(), &location));
                        }

                        let pickups = order_refs.iter().map(|o| o.location()).collect_vec();
                        let (route, _) =
                            calculate_best_route_for_collection(&distance, &pickups, destination);

                        // log::debug!("Collection route: {route:?}");
                        let mut delivery: Vec<&Order> = Vec::with_capacity(order_refs.len());

                        for station in &route {
                            order_refs
                                .iter()
                                .filter(|o| &o.location() == station)
                                .for_each(|pickup_order| {
                                    delivery.push(*pickup_order);

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

                                    train.move_to(station, distance(train.location(), station));
                                });
                        }
                        assert!(delivery.len() == order_refs.len());

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
        Ok(Solution::new(
            moves,
            trains.iter().map(|t| t.traveled_time()).max().unwrap(),
        ))
    }
}
