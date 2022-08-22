use std::fmt::Debug;

use rand::seq::SliceRandom;

use crate::model::{Order, Station};

pub trait OrderSorter: std::fmt::Debug {
    /// Sorts orders in certain order.
    fn sort(&self, orders: &[Order]) -> Vec<Order>;

    /// Whether it always sorts in the same ordera and `sort` call is idempotent.
    fn stable(&self) -> bool {
        true
    }
}

#[derive(Debug)]
pub struct SortOrdersByWeightAsc;
impl OrderSorter for SortOrdersByWeightAsc {
    fn sort(&self, orders: &[Order]) -> Vec<Order> {
        let mut v = orders.to_vec();
        v.sort_by_key(|o| o.weight());
        v
    }
}
#[derive(Debug)]
pub struct SortOrdersByWeightDesc;
impl OrderSorter for SortOrdersByWeightDesc {
    fn sort(&self, orders: &[Order]) -> Vec<Order> {
        let mut v = orders.to_vec();
        v.sort_by_key(|o| o.weight());
        v.reverse();
        v
    }
}

#[derive(Debug)]
pub struct SortOrdersByNameAsc;
impl OrderSorter for SortOrdersByNameAsc {
    fn sort(&self, orders: &[Order]) -> Vec<Order> {
        let mut v = orders.to_vec();
        v.sort_by_key(|o| o.name().to_owned());
        v
    }
}

#[derive(Debug)]
pub struct SortOrdersByNameDesc;
impl OrderSorter for SortOrdersByNameDesc {
    fn sort(&self, orders: &[Order]) -> Vec<Order> {
        let mut v = orders.to_vec();
        v.sort_by_key(|o| o.name().to_owned());
        v.reverse();
        v
    }
}

pub struct SortOrdersByDistanceAsc<F> {
    distance: F,
}

impl<F> SortOrdersByDistanceAsc<F> {
    pub fn new(distance: F) -> Self {
        Self { distance }
    }
}

impl<F> Debug for SortOrdersByDistanceAsc<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SortOrdersByDistanceAsc").finish()
    }
}

impl<F> OrderSorter for SortOrdersByDistanceAsc<F>
where
    F: Fn(&Station, &Station) -> u32,
{
    fn sort(&self, orders: &[Order]) -> Vec<Order> {
        let mut v = orders.to_vec();
        v.sort_by_key(|o| (self.distance)(&o.location(), &o.destination()));
        v
    }
}

pub struct SortOrdersByDistanceDesc<F> {
    distance: F,
}

impl<F> SortOrdersByDistanceDesc<F> {
    pub fn new(distance: F) -> Self {
        Self { distance }
    }
}

impl<F> Debug for SortOrdersByDistanceDesc<F> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SortOrdersByDistanceDesc").finish()
    }
}

impl<F> OrderSorter for SortOrdersByDistanceDesc<F>
where
    F: Fn(&Station, &Station) -> u32,
{
    fn sort(&self, orders: &[Order]) -> Vec<Order> {
        let mut v = orders.to_vec();
        v.sort_by_key(|o| (self.distance)(&o.location(), &o.destination()));
        v.reverse();
        v
    }
}
#[derive(Debug)]
pub struct DoNotSortOrders;
impl OrderSorter for DoNotSortOrders {
    fn sort(&self, orders: &[Order]) -> Vec<Order> {
        orders.to_vec()
    }
}

#[derive(Debug)]
pub struct SortOrdersRandomly;
impl OrderSorter for SortOrdersRandomly {
    fn sort(&self, orders: &[Order]) -> Vec<Order> {
        let mut v = orders.to_vec();
        v.shuffle(&mut rand::thread_rng());
        v
    }

    fn stable(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::{OrderSorter, SortOrdersRandomly};

    #[test]
    fn test_random_sorter() {
        let orders = [
            ("a", 1, "b", "c").into(),
            ("b", 2, "c", "d").into(),
            ("c", 3, "d", "e").into(),
            ("d", 4, "e", "f").into(),
            ("e", 5, "f", "g").into(),
        ];

        // It is highly unlikely that the same order will be returned twice.
        assert!(orders != SortOrdersRandomly.sort(&orders).as_slice());
    }
}
