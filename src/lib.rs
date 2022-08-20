#![feature(iter_intersperse)]

mod input;
mod model;
mod network;
mod output;

use std::collections::VecDeque;

use anyhow::bail;
use model::{Order, Station, Train};
use network::Network;
use output::Move;

pub use crate::input::Input;
pub use crate::output::Output;

pub fn solve(input: &Input) -> anyhow::Result<Output> {
    let network = Network::from(input);

    let trains = input.trains().to_vec();

    let orders = input.orders().to_vec();

    let mut orders_sorted_by_weight_asc = orders.clone();
    orders_sorted_by_weight_asc.sort_by_key(|o| o.weight());

    let mut orders_sorted_by_weight_desc = orders_sorted_by_weight_asc.clone();
    orders_sorted_by_weight_desc.reverse();

    let mut orders_sorted_by_dist_desc = orders.clone();
    orders_sorted_by_dist_desc.sort_by_key(|o| network.distance(&o.location(), &o.destination()));
    [
        orders,
        orders_sorted_by_dist_desc,
        orders_sorted_by_weight_asc,
        orders_sorted_by_weight_desc,
    ]
    .into_iter()
    .map(|orders| deliver_with_closest_train(&network, trains.clone(), orders))
    .collect::<Result<Vec<Output>, _>>()?
    .into_iter()
    // .inspect(|o| println!("Total time: {}", o.total_time()))
    .min_by_key(|o| o.total_time())
    .ok_or_else(|| anyhow::anyhow!("No best solution?"))
}

// Goes through orders, looking up the closest train that can take the package,
// issuing two moves: from train position to package location, from package location to destination.
fn deliver_with_closest_train(
    network: &Network,
    trains: Vec<Train>,
    orders: Vec<Order>,
) -> anyhow::Result<Output> {
    let mut moves = Vec::new();

    let mut orders = VecDeque::from(orders);
    let mut trains = VecDeque::from(trains);

    while let Some(order) = orders.pop_front() {
        // Find the closest available train.
        let pickup_station = order.location();
        let destination_station = order.destination();
        let delivery_distance = network.distance(&pickup_station, &destination_station);

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
            find_closest_train(network, &trains, &pickup_station, order.weight())
        {
            // Remove train from the idle pool.
            let train = trains.remove(idx).expect("Train removed");

            let distance = network.distance(train.location(), &pickup_station);

            log::debug!(
                "TRAIN {} from {} to {}, departure={}, arrival={}",
                train.name(),
                train.location().name(),
                pickup_station.name(),
                train.traveled_time(),
                available_at
            );

            let train = if train.location() != &pickup_station {
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
            } else {
                train
            };

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

            let train = train.move_to(&destination_station, delivery_distance);

            // Return train to the idle pool with updated time.
            trains.push_back(train);
        } else {
            bail!(
                "There is no train that can deliver an order because it is too big, order={}, weight={}",
                order.name(),
                order.weight()
            );
        }
    }

    Ok(Output::new(
        moves,
        trains.iter().map(Train::traveled_time).max().unwrap(),
    ))
}

/// Looks up a train with the nearest arrival time to `location` with at least `min_capacity`.
/// It can only fail if there is no train with required capacity at all.
fn find_closest_train(
    network: &Network,
    trains: &VecDeque<Train>,
    location: &Station,
    min_capacity: u32,
) -> Option<(usize, u32)> {
    // Sort trains by arrival time that is traveled time + travel time to location.
    trains
        .iter()
        .enumerate()
        // Only trains with enough capacity.
        .filter(|(_, train)| train.capacity() >= min_capacity)
        // Calculate availability
        .map(|(index, train)| {
            (
                index,
                train.traveled_time() + network.distance(train.location(), location),
            )
        })
        .min_by_key(|(_, time)| *time)
}

#[cfg(test)]
mod tests {
    use crate::{solve, Input};

    static SIMPLE_INPUT: &str = include_str!("data/simple.txt");

    #[test]
    fn test_solve_simple_input() {
        let input = Input::try_from(SIMPLE_INPUT).expect("Test input");
        let solution = solve(&input).expect("Solve simple input");
        assert_eq!(solution.total_time(), 70);
        assert_eq!(
            &solution.to_string(),
            indoc::indoc! {"
            W=0, T=Q1, N1=B, P1=[], N2=A, P2=[]
            W=30, T=Q1, N1=A, P1=[K1], N2=B, P2=[]
            W=60, T=Q1, N1=B, P1=[], N2=C, P2=[K1]
        "}
        );
    }
}
