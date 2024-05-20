use std::io::{self,Write};
use std::collections::HashMap;

use ansi_term::Colour;

use crate::cli::get_input_filename;
use crate::graphe::reader::read_osm;
use crate::routing::location::{Address,get_location_from_nominatim, nearest_node};
use crate::routing::dijkstra::shortest_path;

pub mod cli;
pub mod graphe;
pub mod routing;


pub fn main() {
    let filename = get_input_filename();

    let g = read_osm( filename.as_str() );

    loop {
        print!( "{}", Colour::Yellow.paint("> " ) );
        io::stdout().flush().unwrap();
        let mut raw_input = String::new();
        match io::stdin().read_line(&mut raw_input) {
            Ok(_n) => {
                let input: Vec<&str> = raw_input.trim().split( ' ' ).collect() ;
                match input[0] {
                    "exit" | "quit" => { break; },
                    "show" => {
                        match input[1] {
                            "node" | "peak" => {
                                let collect = convert_vec( input );
                                print_elts( &g.tnodes, &collect );
                            },
                            "way" | "bow" => {
                                let collect = convert_vec( input );
                                print_elts( &g.tways, &collect );
                            },
                            "nodes" | "peaks" => {
                                let collect = rand_elts( &g.tnodes  );
                                print_elts( &g.tnodes, &collect );
                              },
                            "ways" | "bows" => {
                                let collect = rand_elts( &g.tways );
                                print_elts( &g.tways, &collect );
                              },
                            _ => { println!( "{}", Colour::Red.paint("Choix non valide." ) ); },
                        }
                    },
                    "info" => {
                        println!( "{}", g );
                    },
                    "locate" => {
                        let addr: Vec<&str> = (&input[1..]).to_vec();
                        let ad: Address =  Address { ad: addr.join( " " ) };
                        println!("\n{} :", Colour::Yellow.paint( &ad.ad ) );
                        match get_location_from_nominatim( &ad ) {
                            Ok(lv) => {
                                for l in &lv {
                                    let mut v: Vec<i64> = Vec::new();
                                    v.push( l.osm_id );
                                    match l.osm_type.as_str() {
                                        "way" => { print_elts( &g.tways, &v ); },
                                        "node" => { print_elts( &g.tnodes, &v ); },
                                        _ => { println!("{} => {}", Colour::Red.paint("type non géré"), l ); },
                                    }
                                }
                            },
                            Err(e) => { println!("Erreur {}", e); },
                        };
                    },
                    "nearest" => {
                        // nearest 48.44725 -2.86572 --> Pascal&Nathalie (2345943396 : 48.4471493 , -2.8655416 )
                        // nearest 48.40631 -2.81467 --> garage (10748130358 : 48.4063898 , -2.8150775 )
                        // nearest 48.40627 -2.81457 --> cuisine (10748130360 : 48.4060126 , -2.8146323 )
                        // nearest 48.40672 -2.81433 --> Maryse (4779385124 : 48.4067937 , -2.8145653 )
                        // nearest 48.41119 -2.81940 --> Pharmacie Plaintel (7194631845 : 48.411183 , -2.8197704 )
                        // nearest 48.34743 -2.75695 --> Parmacie Ploeuc (10048845537 : 48.3473733 , -2.7570492 )
                        let ( id, dist ) = nearest_node( input[1].parse::<f64>().unwrap(), input[2].parse::<f64>().unwrap(), &g.tnodes );
                        let distance = format!( "{:.2}", dist );
                        println!( "le point {} est le plus proche à {} m", Colour::Blue.paint( id.to_string() ), Colour::Green.paint( distance ) );
                    },
                    "route" => {
                        // route 10748130360 4779385124 : cuisine-maryse => 360m (361m osm)
                        // route 10748130358 4779385124 : garage-maryse => 82m (83m osm)
                        // route 10748130358 7194631845 : garage-pharmacie Plaintel => 679m (679m osm)
                        // route 10748130358 10048845537 : garage-pharmacie ploeuc => 8990m (9km osm)
                        // route 10748130358 2345943396 : garage-Pascal&Nathalie => 10523m (9km osm) ???
                        match shortest_path( &g.get_directed(), input[1].parse::<i64>().unwrap(), input[2].parse::<i64>().unwrap() ) {
                            Some(l) => { println!( "le plus court chemin est de {} m", Colour::Green.paint( format!("{:.2}", l) ) ); },
                            None => { println!( "impossible  de trouver un chemin"); },
                        }

                    }
                    &_ => {
                        println!( "{} : {}", input[0], Colour::Red.paint("Commande inconnue") );
                    },
                }
            }
            Err(error) => println!("Erreur: {error}"),
        }
    };
}


///
/// convertir la liste d'id nodes ou ways (de type str) en i64
///
fn convert_vec( mut v: Vec<&str> ) -> Vec<i64> {
    let mut collect: Vec<i64> = Vec::new();

    if v.len() > 2 {
        for i in v.split_off(2).iter() {
            collect.push( i.parse::<i64>().unwrap() );
        }
    } else {
        println!( "{}\nuse 'show nodes' cde instead", Colour::Red.paint("need one or more node_id." ) );
    }
    collect
}



///
/// fournir une liste de 5 ways ou node aléatoire
///
fn rand_elts<T>(bt: &HashMap<i64,T>) -> Vec<i64> {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    let mut keys: Vec<i64> = Vec::new();
    for (k, _) in bt.iter() { keys.push( *k ); }

    let max = keys.len();
    let mut i = 0_i32;
    let mut collect: Vec<i64> = Vec::new();
    while i < 5 {
        let nb = rng.gen_range( 0 .. max );
        collect.push( *keys.get( nb ).unwrap() );
        i += 1;
    }
    collect
}


///
/// afficher les élèments de la liste (way ou node)
///
fn print_elts<T: std::fmt::Display>( bt: &HashMap<i64,T>, indexes: &Vec<i64>  ) {
    for i in indexes.iter() {
        match bt.get( &i ) {
            Some(s) => { println!( "{}\n{}", Colour::Blue.paint(i.to_string()), s ); },
            None => { println!( "{}", Colour::Red.paint("id not found") ); }
        }
    }
}
