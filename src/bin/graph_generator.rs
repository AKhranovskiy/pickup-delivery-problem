use itertools::Itertools;
use petgraph::algo::connected_components;
use petgraph::prelude::UnGraph;
use petgraph::visit::EdgeRef;
use rand::Rng;

fn main() -> anyhow::Result<()> {
    let config = Config::new();
    let mut pairs = (0..config.nodes)
        .tuple_combinations()
        .collect::<Vec<(usize, usize)>>();

    let s = pairs.len();

    let mut random = rand::thread_rng();
    for n in ((s - config.edges)..s).rev() {
        let x = random.gen_range(0..n);
        pairs.swap(x, n);
    }
    let edges = pairs.iter().rev().take(config.edges).collect::<Vec<_>>();

    let mut graph = UnGraph::<String, usize>::new_undirected();

    let node_indices = (0..config.nodes)
        .map(|i| graph.add_node(format!("S{i}")))
        .collect::<Vec<_>>();

    edges.iter().for_each(|(a, b)| {
        graph.add_edge(node_indices[*a], node_indices[*b], random.gen_range(1..100));
    });

    assert_eq!(1, connected_components(&graph));

    let trains = (0..random.clone().gen_range(1..node_indices.len()))
        .map(|_| node_indices[random.clone().gen_range(0..node_indices.len())])
        .enumerate()
        .map(|(index, node)| {
            let name = format!("T{}", index);
            let capacity = random.clone().gen_range(1..10_u32);
            let location = format!("S{}", node.index());

            let s = graph.node_weight_mut(node).expect("Node name");
            *s = format!("{s} {name}[{capacity}]");

            (name, capacity, location)
        })
        .collect_vec();

    let max_capacity = trains
        .iter()
        .map(|(_, capacity, _)| *capacity)
        .max()
        .unwrap();

    let orders = (0..random.clone().gen_range(1..node_indices.len()))
        .map(|_| node_indices[random.clone().gen_range(0..node_indices.len())])
        .enumerate()
        .map(|(index, node)| {
            let name = format!("K{}", index);
            let weight = random.clone().gen_range(1..max_capacity);
            let location = format!("S{}", node.index());
            let destination = format!(
                "S{}",
                node_indices[random.clone().gen_range(0..node_indices.len())].index()
            );

            let s = graph.node_weight_mut(node).expect("Node name");
            *s = format!("{s} {name}[{weight} {destination}]");

            (name, weight, location, destination)
        })
        .collect_vec();

    let nodes = graph
        .node_indices()
        .map(|i| format!("S{}", i.index()))
        .collect::<Vec<_>>();

    let edges = graph
        .edge_references()
        .map(|e| {
            let name = format!("E{}", e.id().index());
            let source = format!("S{}", e.source().index());
            let target = format!("S{}", e.target().index());
            let weight = e.weight();
            (name, source, target, weight)
        })
        .collect::<Vec<_>>();

    let mut buffer = String::with_capacity(
        nodes.len() * 3 + edges.len() * 9 + orders.len() * 8 + trains.len() * 6,
    );

    buffer.push_str(format!("{}\n", nodes.len()).as_str());
    buffer.push_str(format!("{}\n\n", nodes.join("\n")).as_str());

    buffer.push_str(format!("{}\n", edges.len()).as_str());
    buffer.push_str(
        Itertools::intersperse(
            edges
                .iter()
                .map(|(name, weight, source, target)| format!("{name},{weight},{source},{target}")),
            "\n".to_owned(),
        )
        .collect::<String>()
        .as_str(),
    );
    buffer.push_str("\n\n");

    buffer.push_str(format!("{}\n", orders.len()).as_str());
    buffer.push_str(
        Itertools::intersperse(
            orders.iter().map(|(name, weight, location, destination)| {
                format!("{name},{weight},{location},{destination}")
            }),
            "\n".to_owned(),
        )
        .collect::<String>()
        .as_str(),
    );
    buffer.push_str("\n\n");

    buffer.push_str(format!("{}\n", trains.len()).as_str());
    buffer.push_str(
        Itertools::intersperse(
            trains
                .iter()
                .map(|(name, capacity, location)| format!("{name},{capacity},{location}")),
            "\n".to_owned(),
        )
        .collect::<String>()
        .as_str(),
    );
    buffer.push('\n');

    println!("{}", buffer);
    Ok(())
}

struct Config {
    pub nodes: usize,
    pub edges: usize,
}

impl Config {
    fn new() -> Self {
        let mut args = std::env::args().skip(1).collect::<Vec<_>>();
        args.reverse();

        let nodes = args
            .pop()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(30);

        let edges = args
            .pop()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(100);

        let max_edges = (nodes - 1) * nodes / 2;

        Self {
            nodes,
            edges: edges.min(max_edges),
        }
    }
}
