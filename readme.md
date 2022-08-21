# Pick and Delivery problem

In Pickup and Delivery problems vehicles have to transport loads from origins to destinations without transshipment at intermediate locations.

Given a network of stations and roads as undirected graph, list of orders on stations with destinations and weights, and list of trains of limited capacity, the program tries to find the best solution to deliver all orders with minimal time.


## Dependencies

**Requires Nightly (1.65)** because of few unstable features.


## Description

Project contains two programs `pdp` that solves the problem, and `graph-generator` that generates an input.

### `pdp`

```
cargo run --release --bin pdp -- src/data/generated.extralarge.1.txt
```

`pdp` reads an input from given file and tries to find an optimal solution using few algorithms:

- `NearestTrainOrderCollectionAlgorithm` groups orders by destination, then looks up for the nearest train to collect orders and deliver the destination via optimal route.
- `NearestTrainOrderDistributionAlgorithm` groups orders by location, then looks up for the nearest train to pickup all orders and deliver to destinations via optimal route.
- `NearestTrainSingleOrderAlgorithm` looks up the nearest train to deliver an order to the destination.

The order list is sorted in various ways for each algorithm to increase chances to find optimal solution.
There are "no-sort" that does not change the order, sorts by properties (weight, name, delivery distance) both ascending and descending, and one random sort that is used multiple times per algorithm.
Note that `NearestTrainOrderCollectionAlgorithm` and `NearestTrainOrderDistributionAlgorithm` sort the order list on their own, so it is useless to apply multiple sorts for them although it can change in future.

The program chooses a solution with the minimal total delivery time, and then outputs a list of moves for each train.

### `graph-generator`

```
cargo run --release --bin graph-generator -- \
 --stations 1000 --edges 10000 --max-edge-weight 1000 -orders 10000 --train 100 --max-train-capacity 20 \
  > src/data/generated.extralarge.2.txt
```

`graph-generator` creates an undirected **connected** graph with given parameters. See program help:
```
USAGE:
    graph_generator [OPTIONS] --stations <STATIONS> --edges <EDGES>

OPTIONS:
        --depot-capacity <DEPOT_CAPACITY>            Max train per station
    -e, --edges <EDGES>                              Number of edges
    -h, --help                                       Print help information
        --max-edge-weight <MAX_EDGE_WEIGHT>          Maximum weight of an edge [default: 100]
        --max-order-weight <MAX_ORDER_WEIGHT>        Max weight of an order [default: 10]
        --max-train-capacity <MAX_TRAIN_CAPACITY>    Max capacity of a train [default: 10]
    -o, --orders <ORDERS>                            Number of orders
    -s, --stations <STATIONS>                        Number of stations
        --station-capacity <STATION_CAPACITY>        Max orders per station
    -t, --trains <TRAINS>                            Number of trains
```


## Authors

- [@AKhranovskiy](https://www.github.com/AKhranovskiy)


## Appendix

Further reading:

- [Margaretha Gansterer, Richard F Hartl, Philipp E. H. Salzmann. Exact solutions for the collaborative pickup and delivery problem](https://d-nb.info/1150750251/34)
- [Zhexi Fu, Joseph Y. J. Chow. The pickup and delivery problem with synchronized en-route transfers for microtransit planning](https://arxiv.org/abs/2107.08218)
