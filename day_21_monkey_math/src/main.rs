use std::collections::HashMap;

use petgraph::data::Build;
use petgraph::graph::NodeIndex;
use petgraph::dot::Dot;
use petgraph::visit::Dfs;
use petgraph::Graph;
use petgraph::algo::toposort;

mod parser;
use parser::parse;

use parser::Formula;

fn basic_graph() {
    let mut graph = Graph::<(), ()>::new();

    graph.extend_with_edges(&[(0, 1)]);

    assert_eq!(graph.node_count(), 2);
    assert_eq!(graph.edge_count(), 1);
}

fn labelled_graph() {
    let mut graph = Graph::new();
    let origin = graph.add_node("Denver");
    let destination_1 = graph.add_node("San Diego");
    let destination_2 = graph.add_node("New York");

    let cost_1 = graph.add_edge(origin, destination_1, 250);
    let cost_2 = graph.add_edge(origin, destination_2, 1099);

    assert_eq!(graph.node_weight(origin).unwrap(), &"Denver");
    assert_eq!(graph[destination_1], "San Diego");
    assert_eq!(graph.edge_weight(cost_1).unwrap(), &250);
    assert_eq!(graph.edge_weight(cost_2).unwrap(), &1099);

    println!("{}", Dot::new(&graph));
}

fn traversal() {
    let mut graph = Graph::<(), (), petgraph::Undirected>::new_undirected();

    // 0(1)(2)3
    graph.extend_with_edges(&[(0, 1), (0, 2), (0, 3)]);

    for start in graph.node_indices() {
        let mut dfs = Dfs::new(&graph, start);

        print!("[{}] ", start.index());

        while let Some(visited) = dfs.next(&graph) {
            print!(" {}", visited.index());
        }

        println!();
    }
}

fn monkey_graph() {
    let mut graph: Graph<&str, i32> = Graph::new();
    let labels = vec![
        "root", "pppw", "sjmn", "dbpl", "cczh", "sllz", "lgvd", "zczc", "ptdq", "humn", "dvpt",
        "lfqf", "ljgn", "drzm", "hmdt",
    ];
    let root = graph.add_node("root");
    let pppw = graph.add_node("pppw");
    let sjmn = graph.add_node("sjmn");
    let root_pppw_edge = graph.add_edge(root, pppw, 0);
    let root_sjmn_edge = graph.add_edge(root, sjmn, 1);

    // 5
    let dbpl = graph.add_node("dbpl");

    // +
    let cczh = graph.add_node("cczh");
    let sllz = graph.add_node("sllz");
    let lgvd = graph.add_node("lgvd");

    let cczh_sllz_edge = graph.add_edge(cczh, sllz, 2);
    let cczh_lgvd_edge = graph.add_edge(cczh, lgvd, 3);

    // 2
    let zczc = graph.add_node("zczc");

    // ptdq: humn - dvpt
    let ptdq = graph.add_node("ptdq");
    let humn = graph.add_node("humn");
    let dvpt = graph.add_node("dvpt");
    let ptdq_humn_edge = graph.add_edge(ptdq, humn, 4);
    let ptdq_dvpt_edge = graph.add_edge(ptdq, dvpt, 5);

    // dvpt: 3
    // let dvpt = graph.add_node("dvpt");

    // lfqf: 4
    let lfqf = graph.add_node("lfqf");

    // humn: 5
    // let humn = graph.add_node("humn");

    // ljgn: 2
    let ljgn = graph.add_node("ljgn");

    // sjmn: drzm * dbpl
    // let sjmn = graph.add_node("sjmn");
    let drzm = graph.add_node("drzm");
    // let dbpl = graph.add_node("dbpl");
    let sjmn_edge = graph.add_edge(sjmn, drzm, 6);
    let sjmn_edge = graph.add_edge(sjmn, dbpl, 7);
    // sllz: 4
    // let sllz = graph.add_node("sllz");

    // pppw: cczh / lfqf
    // let pppw = graph.add_node("pppw");
    let pppw_cczh_edge = graph.add_edge(pppw, cczh, 8);
    let pppw_lfqf_edge = graph.add_edge(pppw, lfqf, 9);

    // lgvd: ljgn * ptdq
    // let ljgn = graph.add_node("ljgn");

    let lgvd_ljgn_edge = graph.add_edge(lgvd, ljgn, 10);
    let lgvd_ptdq_edge = graph.add_edge(lgvd, ptdq, 11);

    // drzm: hmdt - zczc
    // let drzm = graph.add_node("drzm");
    // hmdt: 32
    let hmdt = graph.add_node("hmdt");

    let drzm_hmdt_edge = graph.add_edge(drzm, hmdt, 12);
    let drzm_zczc_edge = graph.add_edge(drzm, zczc, 13);

    println!("{:?}", Dot::new(&graph));

    &graph.reverse();
    let mut dfs = Dfs::new(&graph, root);

    println!("[{:?}] ", root);

    while let Some(visited) = dfs.next(&graph) {
        // visited.index()

        println!("{:?}", labels.get(visited.index()).unwrap());
    }

    println!();

    match toposort(&graph, None) {
        Ok(order) => {
            print!("Sorted ");
            //let mut it:Iterator = order.into();
            // for i in it.reverse() {
                for i in order {
                graph.node_weight(i).map(|weight| {
                    println!("{}", weight);
                    weight
                });
            }
        },
        Err(err) => {
            graph.node_weight(err.node_id()).map(|weight|
            println!("Error graph has a cycle at node {}", weight));
        }
    }
}

fn build_graph(cache: &mut HashMap<String, Formula>) -> Graph<String, ()> {
    let mut nodes: HashMap<String, NodeIndex> = HashMap::new();
    let mut graph:Graph<String, ()> = Graph::new();

    for key in cache.keys() {

        if !nodes.contains_key(key) {
            nodes.insert(key.to_string(), graph.add_node(key.clone()));
        }
        
        match cache.get(key) {
            Some(Formula::N(num)) => {
                println!("Ignoring {key}={num} for now");
            },
            Some(Formula::F(formula)) => {
                let mut parts = (*formula).split(" ");
                let left = parts.next().unwrap();
                let op = parts.next().unwrap();
                let right = parts.next().unwrap();
                println!("Ignoring {op} for {left} {op} {right}");
                
                if !nodes.contains_key(left) {
                    nodes.insert(left.to_string(), graph.add_node(left.to_string()));
                }
                if !nodes.contains_key(right) {
                    nodes.insert(right.to_string(), graph.add_node(right.to_string()));
                }
                let a = nodes.get(key).unwrap();
                let b = nodes.get(left).unwrap();
                let c = nodes.get(right).unwrap();
                graph.add_edge(*a, *b, ());
                graph.add_edge(*a, *c, ());
            },
            None => panic!("Unknown entry {key}")
        }
    }
    println!("{:?}", Dot::new(&graph));
    graph
}

fn main() {
    basic_graph();
    labelled_graph();
    traversal();

    monkey_graph();

    let test_mode = env!("TEST_MODE") == "true";
    let mut cache: HashMap<String, Formula> = HashMap::new();
    parse(if test_mode {
        "src/test_input.txt"
    } else {
        "src/input.txt"
    }, &mut cache);

    let mut graph = build_graph(&mut cache);

    graph.reverse();

    match toposort(&graph, None) {
        Ok(order) => {
            print!("Sorted ");
            //let mut it:Iterator = order.into();
            // for i in it.reverse() {
                for i in order {
                    graph.node_weight(i).map(|weight| {
                        match cache.get(weight) {
                            Some(Formula::N(num)) => {
                                println!("Visit {weight}={num} no action");
                            },
                            Some(Formula::F(formula)) => {
                                
                                let mut parts = formula.split(" ");
                                let left = cache.get(parts.next().unwrap()).expect("left side missing in cache");
                                let op = parts.next().unwrap();
                                let right = cache.get(parts.next().unwrap()).expect("right side missing in cache");

                                match (left, right) {
                                    (Formula::N(left_n), Formula::N(right_n)) => {
                                        println!("Visit {weight} {left_n} {op} {right_n} ");
                                        if weight == "root" {
                                            println!("Dealing with the root... {left_n} {right_n}");
                                            // TODO: find side of the graph with humn
                                            // find value of the other side
                                            // make a math equasion under humn = value - under_value 
                                        }
                                        match op {
                                            "+" => {
                                                cache.insert(weight.clone(), Formula::N(left_n + right_n));
                                            },
                                            "-" => {
                                                cache.insert(weight.clone(), Formula::N(left_n - right_n));
                                            },
                                            "*" => {
                                                cache.insert(weight.clone(), Formula::N(left_n * right_n));
                                            },
                                            "/" => {
                                                cache.insert(weight.clone(), Formula::N(left_n / right_n));
                                            },
                                            _ => panic!("Unknown operator")
                                        }
                                        
                                        
                                        
                                    },
                                    _ => panic!("Expected cache to have numbers, but it did not")
                                }
                                
                            },
                            None => panic!("Cache is missing {weight}")
                        }
                        println!("key={}", weight);
                        weight
                    });
                
            }
        },
        Err(err) => {
            graph.node_weight(err.node_id()).map(|weight|
            println!("Error graph has a cycle at node {}", weight));
        }
    }

    match cache.get("root").unwrap() {
        Formula::N(answer) => println!("Answer is {:?}", answer),
        _ => panic!("Something went wrong")
    }
    

}
