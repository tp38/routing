use std::collections::HashMap;
use std::fmt;
use osmpbf::{DenseNode, Way };
use crate::graphe::elements::{TNodeType, Edge, TNode, TWay};
use crate::routing::distances::distance_haversine;

pub mod reader;
pub mod elements;


pub struct Graph {
    filename: String,
    pub tnodes: HashMap<i64, TNode>,
    pub tways: HashMap<i64, TWay>,
}


impl Graph {
    ///
    /// create new graph from tnodes and tways collections
    ///
    pub fn new( f: String, ip: HashMap<i64, TNode>, ib: HashMap<i64, TWay> ) -> Self {
        Self { filename: f, tnodes: ip, tways: ib }
    }

    ///
    /// retain only tnode that are used by tways
    ///
    pub fn clean(&mut self) {
        for ( _i, e ) in self.tways.iter() {
            let mut count: usize = 0;
            let mut lastid: &i64 = &0;
            for n in e.refs().iter() {
                lastid = n;
                if count == 0 { self.tnodes.get_mut(lastid).expect("n must be in hashmap").set_type( TNodeType::EndNode ); }
                else { self.tnodes.get_mut(lastid).expect("n must be in hashmap").set_type( TNodeType::MiddleNode ); }
                count += 1;
            }
            self.tnodes.get_mut(lastid).expect("n must be in hashmap").set_type( TNodeType::EndNode );
        }
        self.tnodes.retain( |_k, v| v.get_type() != TNodeType::UnUsed );
    }

    ///
    /// make a directed graph that can be used by Dijkstra shortest_path function (see dijkstra.rs)
    ///
    pub fn get_directed(&self) -> HashMap<i64,Vec<Edge>> {
        let mut graph: HashMap<i64,Vec<Edge>> = HashMap::new();
        let mut maxspeed: f64 = 0.0;

        for (_k, w) in self.tways.iter() {
            // pour chacun des segments composant la voie (way)
    	    for (key, v) in w.tags() {
        		if key == "highway" {
        		    match v.as_str() {
                        "motorway" => { maxspeed = 130.0 },
            			"trunk" |
            			"primary" => { maxspeed = 110.0 },
            			"secondary" => { maxspeed = 80.0 },
                        "primary_link" |
                        "trunk_link" =>  { maxspeed = 70.0 },
            			"road" |
                        "tertiary_link" |
                        "secondary_link" |
                        "residential" |
                        "unclassified" |
            			"tertiary"  => { maxspeed = 50.0 },
                        &_ => {}
        	       }
                }
            }
    	    for (key, v) in w.tags() {
        		if key == "maxspeed" {
                    maxspeed = v.parse::<f64>().unwrap();
        		}
    	    }

            for i in 1..w.refs().len() {
                // les id des noeuds
                let start_idx = w.refs()[i-1];
                let end_idx = w.refs()[i];

                // les datas associées
                let start = self.tnodes.get( &start_idx ).expect( "start node must exist in db ... ");
                let end = self.tnodes.get( &end_idx ).expect( "end node must exist in db ... ");

                // calcul de la distance entre les noeuds
                // let d = distance_pythagore(  start.lat(), start.lon(), end.lat(), end.lon() );
                // let d = distance_sinus( start.lat(), start.lon(), end.lat(), end.lon() );
                let d = distance_haversine( start.lat(), start.lon(), end.lat(), end.lon() );
                let t = (maxspeed / 3.6) / d ; // t en secondes

                // on crée un arc vers le nodeid de fin et comprenant la distance calculée
                let normal = Edge{ node: end_idx, distance: d, time: t, cost: 0.0 };
                match graph.get_mut( &start_idx ) {
                    // l'entrée existe : on reajoute à la liste des arcs du noeud considéré
                    Some(v) => { v.push( normal ); },
                    // l'entrée n'existe pas : on ajoute le node avec une nouvelle liste
                    None => { graph.insert( start_idx, vec![ normal ] ); },
                };
                // on traite les voies a double sens en enregistrant l'arc contraire
                if ! w.oneway() {
                    let reverse = Edge{ node: start_idx, distance: d, time: t, cost: 0.0 };
                    match graph.get_mut( &end_idx ) {
                        Some(v) => { v.push( reverse ); },
                        None => { graph.insert( end_idx, vec![ reverse ] ); },
                    };
                }
            }

        }
        graph
    }

}

///
/// display some info on graph
///
impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
        let mut infos: HashMap<String,usize> = HashMap::new();
        for ( _i, w ) in self.tways.iter() {
            for ( k, v ) in w.tags().iter() {
                if k == "highway" {
                    match infos.remove( v ) {
                        Some(ways) => {
                            let nw = ways + 1;
                            infos.insert( v.to_string(), nw );
                        },
                        None => {
                            infos.insert( v.to_string(), 1 );
                        },
                    }
                }
            }
        }

        write!( f, "file : {}\n\ttways : {} , tnodes : {}\n",
            self.filename, self.tways.len(), self.tnodes.len() ).unwrap();
        write!( f, "graph : \n\ttways are :\n" ).unwrap();
        for ( k, v ) in &infos {
            write!( f, "\t{:20} => {:>7}\n", k, v ).unwrap();
        }
        write!(f, "")
    }

}
