mod algorithms;
mod order_sorter;
mod utils;

use std::time::Instant;

pub use algorithms::*;
use kdam::{tqdm, BarExt};
pub use order_sorter::OrderSorter;
pub use order_sorter::*;

use crate::model::{Order, Station, Train};
use crate::Solution;

pub struct Solver<'s, F>
where
    F: Fn(&Station, &Station) -> u32,
{
    algorithms: &'s [&'s dyn Algorithm],
    order_sorters: &'s [&'s dyn OrderSorter],
    distance: F,
}

impl<'s, F> Solver<'s, F>
where
    F: Fn(&Station, &Station) -> u32,
{
    pub fn new(
        algorithms: &'s [&'s dyn Algorithm],
        order_sorters: &'s [&'s dyn OrderSorter],
        distance: F,
    ) -> Self {
        Self {
            algorithms,
            order_sorters,
            distance,
        }
    }

    pub fn solve(
        &self,
        orders: Vec<Order>,
        trains: Vec<Train>,
    ) -> anyhow::Result<Vec<SolverResult>> {
        const UNSTABLE_SORTS: usize = 100;

        let iterations_per_algorithm = self
            .order_sorters
            .iter()
            .map(|s| if s.stable() { 1 } else { UNSTABLE_SORTS })
            .sum::<usize>();

        let mut pb = tqdm!(total = self.algorithms.len() * iterations_per_algorithm);

        let mut results = Vec::with_capacity(self.algorithms.len() * iterations_per_algorithm);

        for &algorithm in self.algorithms {
            for &order_sorter in self.order_sorters {
                let iterations = if order_sorter.stable() {
                    1
                } else {
                    UNSTABLE_SORTS
                };

                for _ in 0..iterations {
                    let now = Instant::now();
                    let solution = algorithm.solve(
                        order_sorter.sort(&orders).clone(),
                        trains.clone(),
                        &self.distance,
                    )?;
                    let result = SolverResult {
                        elapsed: now.elapsed(),
                        algorithm,
                        order_sorter,
                        solution,
                    };
                    results.push(result);
                    pb.update(1);
                }
            }
        }
        results.sort_by_key(|r| r.solution.total_time());
        Ok(results)
    }
}

pub struct SolverResult<'s> {
    pub elapsed: std::time::Duration,
    pub algorithm: &'s dyn Algorithm,
    pub order_sorter: &'s dyn OrderSorter,
    pub solution: Solution,
}
