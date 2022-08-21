mod nearest_train_order_collection;
mod nearest_train_order_distribution;
mod nearest_train_single_order;

pub use nearest_train_order_collection::NearestTrainOrderCollectionAlgorithm;
pub use nearest_train_order_distribution::NearestTrainOrderDistributionAlgorithm;
pub use nearest_train_single_order::NearestTrainSingleOrderAlgorithm;

use crate::model::{Order, Station, Train};
use crate::Solution;

pub trait Algorithm: std::fmt::Debug {
    fn solve(
        &self,
        orders: Vec<Order>,
        trains: Vec<Train>,
        distance: &dyn Fn(&Station, &Station) -> u32,
    ) -> anyhow::Result<Solution>;
}
