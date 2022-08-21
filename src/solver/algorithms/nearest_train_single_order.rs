use std::collections::VecDeque;

use anyhow::bail;

use crate::model::{Order, Station, Train};
use crate::output::Move;
use crate::solver::utils::find_nearest_train;
use crate::solver::Algorithm;
use crate::Solution;

// Goes through orders, looking up the closest train that can take the package,
// issuing two moves: from train position to package location, from package location to destination.
#[derive(Debug)]
pub struct NearestTrainSingleOrderAlgorithm;

impl Algorithm for NearestTrainSingleOrderAlgorithm {
    fn solve(
        &self,
        orders: Vec<Order>,
        trains: Vec<Train>,
        distance: &dyn Fn(&Station, &Station) -> u32,
    ) -> anyhow::Result<Solution> {
        let mut moves = Vec::new();

        let mut orders = VecDeque::from(orders);
        let mut trains = trains;

        while let Some(order) = orders.pop_front() {
            // Find the closest available train.
            let pickup_station = order.location();
            let destination_station = order.destination();
            let delivery_distance = distance(&pickup_station, &destination_station);

            log::debug!(
                "ORDER {}({}) from {} to {}, distance={}",
                order.name(),
                order.weight(),
                pickup_station.name(),
                destination_station.name(),
                delivery_distance
            );

            if order.is_delivered() {
                log::debug!("Order {} is delivered", order.name());
                continue;
            }

            if let Some((idx, available_at)) =
                find_nearest_train(&distance, &trains, &pickup_station, order.weight())
            {
                // Remove train from the idle pool.
                let mut train = trains.remove(idx);

                let distance = distance(train.location(), &pickup_station);

                log::debug!(
                    "TRAIN {} from {} to {}, departure={}, arrival={}",
                    train.name(),
                    train.location().name(),
                    pickup_station.name(),
                    train.traveled_time(),
                    available_at
                );

                if train.location() != &pickup_station {
                    // Move train to location.
                    moves.push(Move::new(
                        train.traveled_time(),
                        train.name().to_owned(),
                        train.location().name().to_owned(),
                        vec![],
                        pickup_station.name().to_owned(),
                        vec![],
                    ));

                    log::debug!("{:?}", moves.last().unwrap());

                    train.move_to(&pickup_station, distance)
                }

                // Pick up order, move to destination, drop order.
                log::debug!(
                    "ORDER {} delivered from {} to {} by {}, departure={}, arrival={}",
                    order.name(),
                    pickup_station.name(),
                    destination_station.name(),
                    train.name(),
                    train.traveled_time(),
                    train.traveled_time() + delivery_distance
                );

                // Move train to location.
                moves.push(Move::new(
                    train.traveled_time(),
                    train.name().to_owned(),
                    train.location().name().to_owned(),
                    vec![order.name().to_owned()],
                    destination_station.name().to_owned(),
                    vec![order.name().to_owned()],
                ));
                log::debug!("{:?}", moves.last().unwrap());

                train.move_to(&destination_station, delivery_distance);

                // Return train to the idle pool with updated time.
                trains.push(train);
            } else {
                bail!(
                "There is no train that can deliver an order because it is too big, order={}, weight={}",
                order.name(),
                order.weight()
            );
            }
        }

        Ok(Solution::new(
            moves,
            trains.iter().map(Train::traveled_time).max().unwrap(),
        ))
    }
}
