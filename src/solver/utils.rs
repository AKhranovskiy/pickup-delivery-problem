use std::iter::once;

use itertools::Itertools;

use crate::model::{Order, Station, Train};

/// Looks up a train with the nearest arrival time to `location` with at least `min_capacity`.
/// It can only fail if there is no train with required capacity at all.
pub fn find_nearest_train(
    distance: &dyn Fn(&Station, &Station) -> u32,
    trains: &[Train],
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
                train.traveled_time() + distance(train.location(), location),
            )
        })
        .min_by_key(|(_, time)| *time)
}

pub fn calculate_best_route_for_distribution(
    distance: &dyn Fn(&Station, &Station) -> u32,
    start: &Station,
    destinations: &[Station],
) -> (Vec<Station>, u32) {
    Itertools::permutations(0..destinations.len(), destinations.len())
        .map(|nodes| {
            (
                nodes.iter().map(|i| destinations[*i].clone()).collect_vec(),
                once(start)
                    .chain(nodes.iter().map(|i| &destinations[*i]))
                    .tuples()
                    .fold(0_u32, |acc, (a, b)| acc + distance(a, b)),
            )
        })
        // .inspect(|(stations, distance)| println!("{stations:?}: {distance}"))
        .min_by_key(|(_, d)| *d)
        .unwrap()
}

pub fn calculate_best_route_for_collection(
    distance: &dyn Fn(&Station, &Station) -> u32,
    stations: &[Station],
    destination: &Station,
) -> (Vec<Station>, u32) {
    let (mut route, distance) =
        calculate_best_route_for_distribution(&distance, destination, stations);
    route.reverse();
    (route, distance)
}

pub fn group_orders_by_location(orders: &[Order]) -> Vec<(Station, u32, Vec<&Order>)> {
    let mut orders = orders.iter().collect_vec();
    orders.sort_by_key(|order| order.location());
    orders
        .group_by(|a, b| a.location() == b.location())
        .map(|group| {
            let location = group[0].location();
            let total_weight = group.iter().map(|o| o.weight()).sum();
            let orders = group.iter().copied().collect_vec();
            (location, total_weight, orders)
        })
        .sorted_by_key(|(_, w, _)| *w)
        .rev()
        .collect_vec()
}

pub fn group_orders_by_destination(orders: &[Order]) -> Vec<(Station, u32, Vec<&Order>)> {
    let mut orders = orders.iter().collect_vec();
    orders.sort_by_key(|order| order.destination());
    orders
        .group_by(|a, b| a.destination() == b.destination())
        .map(|group| {
            let destination = group[0].destination();
            let total_weight = group.iter().map(|o| o.weight()).sum();
            let orders = group.iter().copied().collect_vec();
            (destination, total_weight, orders)
        })
        .sorted_by_key(|(_, w, _)| *w)
        .rev()
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::model::Station;
    use crate::network::Network;
    use crate::solver::utils::{
        calculate_best_route_for_collection, calculate_best_route_for_distribution,
    };
    use crate::Input;

    #[test]
    fn test_calculate_best_route_for_distribution() {
        let input = Input::try_from(include_str!("../data/generated.small.multiload.txt"))
            .expect("Test input");

        let network = Network::from(&input);
        let distance: &dyn Fn(&Station, &Station) -> u32 = &|a, b| network.distance(a, b);

        let orders = input
            .orders()
            .iter()
            .sorted_by_key(|o| o.location())
            .collect_vec();

        let result = orders
            .group_by(|a, b| a.location() == b.location())
            .filter(|g| g.len() > 1)
            .map(|group| {
                let start = group[0].location();
                let destinations = group.iter().map(|o| o.destination()).collect_vec();

                let (route, distance) =
                    calculate_best_route_for_distribution(&distance, &start, &destinations);
                (start, route, distance)
            })
            .collect_vec();

        assert_eq!(result[0], ("N0".into(), vec!["N3".into(), "N1".into()], 6));
        assert_eq!(result[1], ("N1".into(), vec!["N4".into(), "N2".into()], 5));
        assert_eq!(result[2], ("N2".into(), vec!["N0".into(), "N1".into()], 3));
        assert_eq!(
            result[3],
            ("N4".into(), vec!["N4".into(), "N3".into(), "N0".into()], 6)
        );
    }

    #[test]
    fn test_calculate_best_route_for_collection() {
        let input = Input::try_from(include_str!("../data/generated.small.multiload.txt"))
            .expect("Test input");

        let network = Network::from(&input);
        let distance: &dyn Fn(&Station, &Station) -> u32 = &|a, b| network.distance(a, b);

        let orders = input
            .orders()
            .iter()
            .sorted_by_key(|o| o.destination())
            .collect_vec();

        let result = orders
            .group_by(|a, b| a.destination() == b.destination())
            .filter(|g| g.len() > 1)
            .map(|group| {
                let destination = group[0].destination();
                let pickup = group.iter().map(|o| o.location()).collect_vec();

                let (route, distance) =
                    calculate_best_route_for_collection(&distance, &pickup, &destination);
                (destination, route, distance)
            })
            .collect_vec();

        assert_eq!(result[0], ("N0".into(), vec!["N4".into(), "N2".into()], 3));
        assert_eq!(result[1], ("N1".into(), vec!["N0".into(), "N2".into()], 6));
        assert_eq!(result[2], ("N3".into(), vec!["N0".into(), "N4".into()], 2));
        assert_eq!(
            result[3],
            ("N4".into(), vec!["N1".into(), "N3".into(), "N4".into()], 3)
        );
    }
}
