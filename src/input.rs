use anyhow::{anyhow, ensure, Context};

use crate::model::{Edge, Order, Station, Train};

#[derive(Debug)]
pub struct Input {
    stations: Vec<Station>,
    edges: Vec<Edge>,
    orders: Vec<Order>,
    trains: Vec<Train>,
}

impl Input {
    pub fn stations(&self) -> &[Station] {
        &self.stations
    }

    pub fn edges(&self) -> &[Edge] {
        &self.edges
    }

    pub fn trains(&self) -> &[Train] {
        &self.trains
    }

    pub fn orders(&self) -> &[Order] {
        &self.orders
    }
}

impl TryFrom<&str> for Input {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut lines = input.lines();

        let number_of_stations = lines
            .next()
            .ok_or_else(|| anyhow!("No more lines."))
            .and_then(|line| line.parse::<usize>().map_err(|e| anyhow!("{}", e)))
            .context("Parse number of stations")?;

        ensure!(number_of_stations > 1, "There must be an edge (N1,N2)");

        // Cannot use Take because it consumes the iterator. Advance iterator normally instead.
        let stations = (0..number_of_stations)
            .map(|_| lines.next().unwrap_or_default().trim().into())
            .collect::<Vec<Station>>();

        // TODO Validate stations.

        // Skip empty lines.
        let mut lines = lines.skip_while(|line| line.trim().is_empty());

        let number_of_edges = lines
            .next()
            .ok_or_else(|| anyhow!("No more lines."))
            .and_then(|line| line.parse::<usize>().map_err(|e| anyhow!("{}", e)))
            .context("Parse number of edges")?;

        ensure!(number_of_edges > 0, "There must be an edge (N1,N2)");

        // Cannot use Take because it consumes the iterator. Advance iterator normally instead.
        let edges = (0..number_of_edges)
            .map(|_| {
                let mut parts = lines.next().unwrap_or_default().trim().split(',').take(4);
                let name = parts.next().unwrap_or_default().trim();
                let from = parts.next().unwrap_or_default().trim();
                let to = parts.next().unwrap_or_default().trim();
                let distance = parts
                    .next()
                    .unwrap_or_default()
                    .trim()
                    .parse::<u32>()
                    .unwrap_or_default();

                (name, from, to, distance).into()
            })
            .collect::<Vec<Edge>>();

        // TODO Validate edges.

        // Skip empty lines.
        let mut lines = lines.skip_while(|line| line.trim().is_empty());

        let number_of_orders = lines
            .next()
            .ok_or_else(|| anyhow!("No more lines."))
            .and_then(|line| line.parse::<usize>().map_err(|e| anyhow!("{}", e)))
            .context("Parse number of orders")?;

        ensure!(number_of_edges > 0, "There must be an edge (N1,N2)");

        // Cannot use Take because it consumes the iterator. Advance iterator normally instead.
        let orders = (0..number_of_orders)
            .map(|_| {
                let mut parts = lines.next().unwrap_or_default().trim().split(',').take(4);
                let name = parts.next().unwrap_or_default().trim();
                let weight = parts
                    .next()
                    .unwrap_or_default()
                    .trim()
                    .parse::<u32>()
                    .unwrap_or_default();
                let from = parts.next().unwrap_or_default().trim();
                let to = parts.next().unwrap_or_default().trim();

                (name, weight, from, to).into()
            })
            .collect::<Vec<Order>>();

        // TODO Validate orders.

        // Skip empty lines.
        let mut lines = lines.skip_while(|line| line.trim().is_empty());

        let number_of_trains = lines
            .next()
            .ok_or_else(|| anyhow!("No more lines."))
            .and_then(|line| line.parse::<usize>().map_err(|e| anyhow!("{}", e)))
            .context("Parse number of trains")?;

        ensure!(number_of_trains > 0, "There should be a train");

        // Cannot use Take because it consumes the iterator. Advance iterator normally instead.
        let trains = (0..number_of_trains)
            .map(|_| {
                let mut parts = lines.next().unwrap_or_default().trim().split(',').take(4);
                let name = parts.next().unwrap_or_default().trim();
                let capacity = parts
                    .next()
                    .unwrap_or_default()
                    .trim()
                    .parse::<u32>()
                    .unwrap_or_default();
                let location = parts.next().unwrap_or_default().trim();

                (name, capacity, location).into()
            })
            .collect::<Vec<Train>>();

        // TODO Validate trains.

        Ok(Self {
            stations,
            edges,
            orders,
            trains,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::input::Edge;
    use crate::model::{Order, Station, Train};
    use crate::Input;

    static SIMPLE_INPUT: &str = include_str!("data/simple.txt");

    #[test]
    fn test_parse_simple_input() {
        let sut = Input::try_from(SIMPLE_INPUT).expect("Parse simple input");

        assert_eq!(
            sut.stations,
            ["A", "B", "C"]
                .into_iter()
                .map(Station::from)
                .collect::<Vec<_>>()
        );

        assert_eq!(
            sut.edges,
            [("E1", "A", "B", 30), ("E2", "B", "C", 10)]
                .into_iter()
                .map(Edge::from)
                .collect::<Vec<_>>()
        );

        assert_eq!(
            sut.orders,
            [("K1", 5, "A", "C")]
                .into_iter()
                .map(Order::from)
                .collect::<Vec<_>>()
        );

        assert_eq!(
            sut.trains,
            [("Q1", 6, "B")]
                .into_iter()
                .map(Train::from)
                .collect::<Vec<_>>()
        );
    }
}
