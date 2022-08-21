#![feature(iter_intersperse)]
#![feature(slice_group_by)]

mod input;
pub mod model;
mod network;
mod output;
mod solvers;

use network::Network;
use solvers::{
    deliver_order_by_nearest_train_multiload_collection,
    deliver_order_by_nearest_train_multiload_distribution, deliver_with_nearest_train,
};

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

    let mut orders_sorted_by_dist_asc = orders_sorted_by_dist_desc.clone();
    orders_sorted_by_dist_asc.reverse();
    [
        orders,
        orders_sorted_by_dist_asc,
        orders_sorted_by_dist_desc,
        orders_sorted_by_weight_asc,
        orders_sorted_by_weight_desc,
    ]
    .into_iter()
    .flat_map(|orders| {
        [
            deliver_with_nearest_train(&network, trains.clone(), orders.clone()),
            deliver_order_by_nearest_train_multiload_distribution(
                &network,
                trains.clone(),
                orders.clone(),
            ),
            deliver_order_by_nearest_train_multiload_collection(&network, trains.clone(), orders),
        ]
    })
    .collect::<Result<Vec<Output>, _>>()?
    .into_iter()
    .inspect(|o| println!("Total time: {}", o.total_time()))
    .min_by_key(|o| o.total_time())
    .ok_or_else(|| anyhow::anyhow!("No best solution?"))
}

#[cfg(test)]
mod tests {

    use crate::{solve, Input};

    static SIMPLE_INPUT: &str = include_str!("data/simple.txt");

    #[test]
    #[ignore = "reason"]
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
