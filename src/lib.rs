#![feature(iter_intersperse)]
#![feature(slice_group_by)]

mod input;
pub mod model;
mod network;
mod output;
mod solver;

use model::Station;
use network::Network;
use solver::{OrderSorter, Solver, SolverResult};

pub use crate::input::Input;
pub use crate::output::Solution;

pub fn solve(input: &Input) -> anyhow::Result<Solution> {
    let network = Network::from(input);

    let distance: &dyn Fn(&Station, &Station) -> u32 = &|from, to| network.distance(from, to);
    let trains = input.trains().to_vec();
    let orders = input.orders().to_vec();

    let sorters: &[&dyn OrderSorter] = &[
        &solver::DoNotSortOrders,
        &solver::SortOrdersByDistanceAsc::new(distance),
        &solver::SortOrdersByDistanceDesc::new(distance),
        &solver::SortOrdersByNameAsc,
        &solver::SortOrdersByNameDesc,
        &solver::SortOrdersRandomly,
    ];

    let solver = Solver::new(
        &[
            &solver::NearestTrainSingleOrderAlgorithm,
            &solver::NearestTrainOrderDistributionAlgorithm,
            &solver::NearestTrainOrderCollectionAlgorithm,
        ],
        sorters,
        distance,
    );

    let solutions = solver.solve(orders, trains)?;

    print_statistic(&solutions);

    solutions
        .into_iter()
        .next()
        .map(|r| r.solution)
        .ok_or_else(|| anyhow::anyhow!("no solution found"))
}

fn print_statistic(results: &[SolverResult]) {
    for (i, result) in results.iter().enumerate().take(10).rev() {
        println!(
            "{:>2} {:<50?} / {:<50?}  {:>3}ms {:>3}",
            i + 1,
            result.algorithm,
            result.order_sorter,
            result.elapsed.as_millis(),
            result.solution.total_time()
        );
    }
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
