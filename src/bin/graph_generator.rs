#![feature(int_log)]

use std::collections::HashMap;

use clap::Parser;
use itertools::Itertools;
use petgraph::visit::EdgeRef;
use petgraph::{algo::connected_components, prelude::UnGraph};
use petgraph::{Graph, Undirected};
use rand::{distributions::Uniform, prelude::Distribution};
use rand::{thread_rng, Rng};

#[derive(Debug, Parser)]
struct Args {
    /// Number of stations.
    #[clap(short, long)]
    stations: usize,

    /// Number of edges.
    #[clap(short, long)]
    edges: usize,

    /// Maximum weight of an edge.
    #[clap(long, default_value = "100")]
    max_edge_weight: u32,

    /// Number of orders.
    #[clap(short, long)]
    orders: Option<usize>,

    /// Max weight of an order.
    #[clap(long, default_value = "10")]
    max_order_weight: u32,

    /// Max orders per station.
    #[clap(long)]
    station_capacity: Option<usize>,

    /// Number of trains.
    #[clap(short, long)]
    trains: Option<usize>,

    /// Max capacity of a train.
    #[clap(long, default_value = "10")]
    max_train_capacity: u32,

    /// Max train per station.
    #[clap(long)]
    depot_capacity: Option<usize>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let graph = generate_graph(args.stations, args.edges, args.max_edge_weight);

    let stations = graph
        .node_indices()
        .map(|i| station_label(&graph, i))
        .collect_vec();

    let edges = graph
        .edge_references()
        .map(|e| {
            let name = edge_label(e.id().index());
            let source = station_label(&graph, e.source());
            let target = station_label(&graph, e.target());
            let weight = *e.weight();

            format!("{name},{source},{target},{weight}")
        })
        .collect_vec();

    let orders = generate_orders(
        &graph,
        args.orders,
        args.max_order_weight,
        args.station_capacity,
    );

    let trains = generate_trains(
        &graph,
        args.trains,
        args.max_train_capacity,
        args.depot_capacity,
    );

    let mut buffer =
        String::with_capacity(buffer_capacity(&args, &stations, &edges, &orders, &trains));

    [stations, edges, orders, trains]
        .into_iter()
        .for_each(|lines| {
            buffer.push_str(&format!("{}\n", lines.len()));
            buffer.push_str(&format!("{}\n", lines.join("\n")));
            buffer.push('\n');
        });

    println!("{}", buffer);

    Ok(())
}

fn buffer_capacity(
    args: &Args,
    stations: &[String],
    edges: &[String],
    orders: &[String],
    trains: &[String],
) -> usize {
    let st = length(args.stations);
    let ed = length(args.edges);
    let mew = length(args.max_edge_weight as usize);
    let mow = length(args.max_order_weight as usize);
    let mtc = length(args.max_train_capacity as usize);
    stations.len() * (1 + st)
        + edges.len() * (6 + ed + st * 2 + mew)
        + orders.len() * (6 + ed + st * 2 + mow)
        + trains.len() * (4 + ed + st + mtc)
}

// fn get_buffer_capacity()
fn edge_label(index: usize) -> String {
    format!("E{index}")
}

fn length(n: usize) -> usize {
    (n.ilog(10) + 1) as usize
}

fn generate_trains(
    graph: &Graph<usize, u32, Undirected>,
    number_of_trains: Option<usize>,
    max_train_capacity: u32,
    depot_capacity: Option<usize>,
) -> Vec<String> {
    let mut rng = thread_rng();
    let node_count = graph.node_count();
    let number_of_trains = number_of_trains.unwrap_or_else(|| rng.gen_range(1..node_count));
    let depot_capacity = depot_capacity.unwrap_or(usize::MAX);

    let mut depot_load = HashMap::new();

    let dist = Uniform::from(0..node_count);
    rng.clone()
        .sample_iter(dist)
        .map(|mut node| {
            loop {
                let load = depot_load.entry(node).or_insert(0);
                if (*load + 1) > depot_capacity {
                    node = dist.sample(&mut rng.clone());
                    continue;
                }
                *load += 1;
                break;
            }
            graph
                .node_indices()
                .nth(node)
                .expect("{node} must be valid node index")
        })
        .enumerate()
        .map(|(index, node)| {
            let name = train_label(index);
            let capacity = rng.clone().gen_range(1..max_train_capacity);
            let location = station_label(graph, node);

            format!("{name},{capacity},{location}")
        })
        .take(number_of_trains)
        .collect_vec()
}

fn train_label(index: usize) -> String {
    format!("T{index}")
}

fn generate_graph(nodes: usize, edges: usize, max_edge_weight: u32) -> UnGraph<usize, u32> {
    let mut rnd = thread_rng();

    loop {
        let mut graph = UnGraph::new_undirected();

        for n in 0..nodes {
            graph.add_node(n);
        }

        for _ in 0..edges {
            let from = rnd.gen_range(0..nodes);
            let to = rnd.gen_range(0..nodes);
            let weight = rnd.gen_range(1..max_edge_weight);
            graph.add_edge(
                graph
                    .node_indices()
                    .nth(from)
                    .expect("Node index must be valid"),
                graph
                    .node_indices()
                    .nth(to)
                    .expect("Node index must be valid"),
                weight,
            );
        }

        if connected_components(&graph) == 1 {
            return graph;
        }
    }
}

fn generate_orders(
    graph: &Graph<usize, u32, Undirected>,
    number_of_orders: Option<usize>,
    max_order_weight: u32,
    station_capacity: Option<usize>,
) -> Vec<String> {
    let mut rng = thread_rng();
    let node_count = graph.node_count();
    let number_of_orders = number_of_orders.unwrap_or_else(|| rng.gen_range(1..node_count));
    let station_capacity = station_capacity.unwrap_or(usize::MAX);

    let mut station_load = HashMap::new();

    let dist = Uniform::from(0..node_count);
    rng.clone()
        .sample_iter(dist)
        .tuples()
        .map(|(mut src, dst)| {
            loop {
                let load = station_load.entry(src).or_insert(0);
                if (*load + 1) > station_capacity {
                    src = dist.sample(&mut rng.clone());
                    continue;
                }
                *load += 1;
                break;
            }
            (
                graph
                    .node_indices()
                    .nth(src)
                    .expect("{n} must be valid node index"),
                graph
                    .node_indices()
                    .nth(dst)
                    .expect("{m} must be valid node index"),
            )
        })
        .enumerate()
        .map(|(index, (src, dst))| {
            let name = order_label(index);
            let weight = rng.clone().gen_range(1..max_order_weight);
            let location = station_label(graph, src);
            let destination = station_label(graph, dst);
            format!("{name},{weight},{location},{destination}")
        })
        .take(number_of_orders)
        .collect_vec()
}

fn station_label(
    graph: &Graph<usize, u32, Undirected>,
    src: petgraph::stable_graph::NodeIndex,
) -> String {
    format!(
        "N{}",
        *graph
            .node_weight(src)
            .expect("NodeIndex[{src}] must be valid."),
    )
}

fn order_label(index: usize) -> String {
    format!("K{index}")
}

// fn node_label(graph: )
