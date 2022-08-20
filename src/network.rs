use std::collections::HashMap;

use petgraph::algo::floyd_warshall;
use petgraph::prelude::UnGraph;
use petgraph::stable_graph::NodeIndex;

use crate::model::Station;
use crate::Input;

#[derive(Debug)]
pub struct Network<'n> {
    _graph: UnGraph<&'n Station, u32>,
    station_to_index: HashMap<&'n Station, NodeIndex>,
    distances: HashMap<(NodeIndex<u32>, NodeIndex<u32>), u32>,
}

impl<'n> Network<'n> {
    pub fn _distances(&self) -> HashMap<(&Station, &Station), u32> {
        self.distances
            .iter()
            .map(|(&(from, to), &distance)| {
                (
                    (
                        *self._graph.node_weight(from).unwrap(),
                        *self._graph.node_weight(to).unwrap(),
                    ),
                    distance,
                )
            })
            .collect()
    }

    // I assume that all nodes are connected.
    pub fn distance(&self, from: &Station, to: &Station) -> u32 {
        let from = self.station_to_index.get(from).unwrap();
        let to = self.station_to_index.get(to).unwrap();
        *self.distances.get(&(*from, *to)).unwrap()
    }
}

impl<'n> From<&'n Input> for Network<'n> {
    fn from(input: &'n Input) -> Self {
        let mut graph = UnGraph::new_undirected();
        let node_map = input
            .stations()
            .iter()
            .map(|s| (s, graph.add_node(s)))
            .collect::<HashMap<_, _>>();

        input.edges().iter().for_each(|e| {
            graph.add_edge(
                node_map[&e.stations().0],
                node_map[&e.stations().1],
                e.distance(),
            );
            graph.add_edge(
                node_map[&e.stations().1],
                node_map[&e.stations().0],
                e.distance(),
            );
        });

        let distances = floyd_warshall(&graph, |e| *e.weight()).expect("Calcucalte distances");

        Self {
            _graph: graph,
            station_to_index: node_map,
            distances,
        }
    }
}
