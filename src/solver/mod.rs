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
        let mut pb = tqdm!();

        let mut results = Vec::new();

        for &algorithm in self.algorithms {
            if algorithm.sort_sensitive() {
                // Iterate over sorters.
                for &order_sorter in self.order_sorters {
                    let iterations = if order_sorter.stable() { 1 } else { 100 };
                    for _ in 0..iterations {
                        let now = Instant::now();

                        let solution = algorithm
                            .solve(order_sorter.sort(&orders), trains.clone(), &self.distance)
                            .unwrap_or_else(|e| {
                                log::error!("{algorithm:?} / {order_sorter:?}: {e:#?}");
                                Solution::new(vec![], u32::MAX)
                            });

                        results.push(SolverResult {
                            elapsed: now.elapsed(),
                            algorithm,
                            order_sorter: Some(order_sorter),
                            solution,
                        });
                        pb.update(1);
                    }
                }
            } else {
                let now = Instant::now();

                let solution = algorithm
                    .solve(orders.clone(), trains.clone(), &self.distance)
                    .unwrap_or_else(|e| {
                        log::error!("{algorithm:?} / None: {e:#?}");
                        Solution::new(vec![], u32::MAX)
                    });

                results.push(SolverResult {
                    elapsed: now.elapsed(),
                    algorithm,
                    order_sorter: None,
                    solution,
                });

                pb.update(1);
            }
        }

        results.sort_by_key(|r| r.solution.total_time());
        Ok(results)
    }
}

pub struct SolverResult<'s> {
    pub elapsed: std::time::Duration,
    pub algorithm: &'s dyn Algorithm,
    pub order_sorter: Option<&'s dyn OrderSorter>,
    pub solution: Solution,
}
