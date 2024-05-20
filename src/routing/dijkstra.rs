use std::cmp::Ordering;
use std::collections::{HashMap,BTreeMap,BinaryHeap};
use crate::graphe::elements::Edge;

#[derive(Copy, Clone)]
pub struct State {
    cost: f64,
    position: i64,
}

// impl State {
//     pub fn cost(&self) -> f64 {
//         self.cost
//     }
//
//     pub fn position(&self) -> i64 {
//         self.position
//     }
// }

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost && self.position == other.position
    }
}

impl Eq for State {
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        if other.cost.eq(&self.cost) {
            if other.position.eq(&self.position) { Ordering::Equal }
            else if other.position.lt(&self.position) { Ordering::Less }
            else { Ordering::Greater }
        }
        else if other.cost.lt( &self.cost ) { Ordering::Less }
        else { Ordering::Greater}
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
pub fn simple_shortest_path(adj_list: &HashMap<i64, Vec<Edge>>, start: i64, goal: i64) -> Option<f64> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: HashMap<i64,f64> = HashMap::new();
    for ( k, _v ) in adj_list.iter() { dist.insert( *k, f64::MAX ); }

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    match dist.get_mut( &start ) {
        Some(n) => {
            *n = 0.0;
            heap.push(State { cost: 0.0, position: start });

            // Examine the frontier with lower cost nodes first (min-heap)
            while let Some( State { cost, position } ) = heap.pop() {
                // Alternatively we could have continued to find all shortest paths
                if position == goal { return Some(cost); }

                // Important as we may have already found a better way
                if cost > *dist.get( &position ).unwrap() { continue; }

                // For each node we can reach, see if we can find a way with
                // a lower cost going through this node
                for edge in adj_list.get( &position ).unwrap() {
                    let next = State { cost: cost + edge.distance, position: edge.node };

                    // If so, add it to the frontier and continue
                    match dist.get_mut( &next.position ) {
                        Some(n) => {
                            if next.cost < *n {
                                heap.push(next);
                                // Relaxation, we have now found a better way
                                *n = next.cost;
                            }
                        },
                        None => {
                            println!("{:?}", edge );
                        }
                    }
                }
            }
        },
        None => { println!( "start node must be in the graph" ); },
    }
    // Goal not reachable
    None
}

pub struct Seg {
    dist: f64,
    old: i64
}

impl Seg {
    pub fn old(&self) -> i64 {
        self.old
    }

    pub fn dist(&self) -> f64 {
        self.dist
    }

    pub fn set_dist(&mut self, n: f64) {
        self.dist = n;
    }

    pub fn set_old(&mut self, o: i64) {
        self.old = o;
    }
}


// Dijkstra's shortest path algorithm.
// with path return
pub fn shortest_path(mode: &str, adj_list: &HashMap<i64, Vec<Edge>>, start: i64, goal: i64) -> Option<BTreeMap<i64,i64>> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: HashMap<i64,Seg> = HashMap::new();
    for ( k, _v ) in adj_list.iter() { dist.insert( *k, Seg { dist: f64::MAX, old: i64::MAX } ); }

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    match dist.get_mut( &start ) {
        Some(n) => {
            *n = Seg {dist: 0.0, old: 0};
            heap.push( State { cost: 0.0, position: start } );

            // Examine the frontier with lower cost nodes first (min-heap)
            while let Some( State { cost, position } ) = heap.pop() {
                if position == goal {
                    let mut nodes: BTreeMap<i64,i64> = BTreeMap::new();
                    let mut cur = goal;
                    loop{
                        match dist.get(&cur) {
                            Some(s) => {
                                if s.old() == 0 {
                                    nodes.entry( (s.dist() * 100.0) as i64 )
                                        .and_modify(|c| *c = cur )
                                        .or_insert( cur );
                                    break;
                                }
                                else {
                                    nodes.entry( (s.dist() * 100.0) as i64 )
                                        .and_modify(|c| *c = cur )
                                        .or_insert( cur );
                                    cur = s.old();
                                }
                            },
                            None => { panic!( "must be in db" ); }
                        }
                    }
                    return Some(nodes);
                }

                // Important as we may have already found a better way
                if cost > dist.get( &position ).unwrap().dist() { continue; }

                // For each node we can reach, see if we can find a way with
                // a lower cost going through this node
                for edge in adj_list.get( &position ).unwrap() {
                    let c = if mode == "time" { edge.time } else { edge.distance };
                    let next = State { cost: cost + c, position: edge.node };

                    // If so, add it to the frontier and continue
                    match dist.get_mut( &next.position ) {
                        Some(n) => {
                            if next.cost < n.dist() {
                                heap.push(next);
                                // Relaxation, we have now found a better way
                                n.set_dist( next.cost );
                                n.set_old( position )
                            }
                        },
                        None => {
                            println!("{:?}", edge );
                        }
                    }
                }
            }
        },
        None => { println!( "start node must be in the graph" ); },
    }
    // Goal not reachable
    None
}


#[cfg(test)]
mod dijkstra_tests {
    use super::*;


    #[test]
    fn test_shortest_path() {
        // This is the directed graph we're going to use.
        // The node numbers correspond to the different states,
        // and the edge weights symbolize the cost of moving
        // from one node to another.
        // Note that the edges are one-way.
        //
        //                  7
        //          +-----------------+
        //          |                 |
        //          v   1        2    |  2
        //          0 -----> 1 -----> 3 ---> 4
        //          |        ^        ^      ^
        //          |        | 1      |      |
        //          |        |        | 3    | 1
        //          +------> 2 -------+      |
        //           10      |               |
        //                   +---------------+
        //
        // The graph is represented as an adjacency list where each index,
        // corresponding to a node value, has a list of outgoing edges.
        // Chosen for its efficiency.
        // let graph = vec![

        let mut graph: HashMap<i64,Vec<Edge>> = HashMap::new();
        graph.insert(0,
                vec![Edge { node: 2, cost: 10.0 },
                     Edge { node: 1, cost: 1.0 }] );
        graph.insert(1,
                vec![Edge { node: 3, cost: 2.0 }] );
        graph.insert(2,
                vec![Edge { node: 1, cost: 1.0 },
                     Edge { node: 3, cost: 3.0 },
                     Edge { node: 4, cost: 1.0 }] );
        graph.insert(3,
                vec![Edge { node: 0, cost: 7.0 },
                     Edge { node: 4, cost: 2.0 }] );
        graph.insert(4,
                vec![] );

        assert_eq!(simple_shortest_path(&graph, 0, 1), Some(1.0));
        assert_eq!(simple_shortest_path(&graph, 0, 3), Some(3.0));
        assert_eq!(simple_shortest_path(&graph, 3, 0), Some(7.0));
        assert_eq!(simple_shortest_path(&graph, 0, 4), Some(5.0));
        assert_eq!(simple_shortest_path(&graph, 5, 2), None);
        assert_eq!(simple_shortest_path(&graph, 4, 0), None);
    }
}
