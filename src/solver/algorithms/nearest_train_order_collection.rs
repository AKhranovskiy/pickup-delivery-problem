use anyhow::bail;
use itertools::Itertools;

use crate::model::{Order, Train};
use crate::output::Move;
use crate::solver::utils::{
    calculate_best_route_for_collection, find_nearest_train, group_orders_by_destination,
};
use crate::solver::Algorithm;
use crate::Solution;

#[derive(Debug)]
pub struct NearestTrainOrderCollectionAlgorithm;

impl Algorithm for NearestTrainOrderCollectionAlgorithm {
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
                    let pickups = order_refs.iter().map(|o| o.location()).collect_vec();
                    let (mut route, _) =
                        calculate_best_route_for_collection(&distance, &pickups, destination);

                    route.push(destination.clone());

                    // Find the nearest train to the beginning of the route.
                    let location = route.first().unwrap();

                    if let Some((train_index, available_at)) =
                        find_nearest_train(&distance, &trains, location, *w)
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

                            train.move_to(location, distance(train.location(), location));
                        }

                        let mut delivery: Vec<&Order> = Vec::with_capacity(order_refs.len());

                        for (current, next) in route.iter().tuple_windows() {
                            let mut to_collect = order_refs
                                .iter()
                                .filter(|o| &o.location() == current)
                                .copied()
                                .collect_vec();

                            delivery.append(&mut to_collect);

                            let names = delivery.iter().map(|o| o.name().to_owned()).collect_vec();

                            assert!(train.location() == current);

                            log::debug!(
                                "TRAIN {} from={} load={:?} to={} unload={:?} distance={}",
                                train.name(),
                                train.location().name(),
                                names,
                                next.name(),
                                names,
                                distance(train.location(), next)
                            );

                            moves.push(Move::new(
                                train.traveled_time(),
                                train.name().to_owned(),
                                train.location().name().to_owned(),
                                names.clone(),
                                next.name().to_owned(),
                                names,
                            ));

                            log::debug!("{:?}", moves.last().unwrap());

                            train.move_to(next, distance(train.location(), next));
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
