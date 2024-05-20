use std::io::{self,Write};
use std::fs::File;
use std::collections::HashMap;

use ansi_term::Colour;
use gpx::{Gpx,GpxVersion,Metadata,Track,TrackSegment, Waypoint,write};
use geo_types::{Point, Rect, coord};


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
                        // nearest 48.51973 -2.78808 --> Dr Smau ( 2000599137 : 48.5197604 , -2.7879812000000004 )
                        // nearest 48.49618 -2.68939 --> Denis Rebours ( 2971599465 : 48.496328000000005 , -2.6892531)
                        let ( id, dist ) = nearest_node( input[1].parse::<f64>().unwrap(), input[2].parse::<f64>().unwrap(), &g.tnodes );
                        let distance = format!( "{:.2}", dist );
                        println!( "le point {} est le plus proche à {} m", Colour::Blue.paint( id.to_string() ), Colour::Green.paint( distance ) );
                    },
                    "route" => {
                        // route time 10748130360 4779385124 : cuisine-maryse => 359.85m (361m osm)
                        // route distance 10748130358 4779385124 : garage-maryse => 82.37m (83m osm)
                        // route time 10748130358 7194631845 : garage-pharmacie Plaintel => 678.56m (679m osm)
                        // route distance 10748130358 10048845537 : garage-pharmacie ploeuc => 8989.94m (9km osm)
                        // route time 10748130358 2345943396 : garage-Pascal&Nathalie => 10522.86m (9km osm) ???
                        // route distance 10748130358 2000599137 : garage-Dr_Smau => 15228.37m (16km osm)
                        // route time 10748130358 2971599465 : garage-Denis_Rebours => 17313.70m (18km osm)
                        match shortest_path( input[1], &g.get_directed(), input[2].parse::<i64>().unwrap(), input[3].parse::<i64>().unwrap() ) {
                            Some(bt) => {
                                for (k, v) in bt.iter() {
                                    println!( "{} : ", Colour::Yellow.paint( format!( "{} m", (*k as f64 / 100.0) ) ) );
                                    print_elts( &g.tnodes, &vec![*v] );
                                    // affichage des way id et des noms de rue
                                    match g.tnodes.get( v ) {
                                        Some(n) => {
                                            for id in n.ways() {
                                                match g.tways.get( &id ) {
                                                    Some(w) => {
                                                        for (t,val) in w.tags() {
                                                            if ( t == "name" ) | ( t == "ref" ) {
                                                                println!( "\t{} : {}", id, val );
                                                                break;
                                                            }
                                                        }
                                                    },
                                                    None => {
                                                        println!( "way id {} must be in db", id );
                                                    },
                                                }
                                            }
                                        },
                                        None => {
                                            println!( "node id {} must be in db", v );
                                        }
                                    }
                                    // fin affichage
                                }
                            },
                            None => { println!( "impossible  de trouver un chemin"); },
                        }
                    }
                    "gpx" => {
                        // gpx distance 10748130360 4779385124 : cuisine-maryse => 359.85m (361m osm)
                        // gpx time 10748130358 4779385124 : garage-maryse => 82.37m (83m osm)
                        // gpx distance 10748130358 7194631845 : garage-pharmacie Plaintel => 678.56m (679m osm)
                        // gpx time 10748130358 10048845537 : garage-pharmacie ploeuc => 8989.94m (9km osm)
                        // gpx distance 10748130358 2345943396 : garage-Pascal&Nathalie => 10522.86m (9km osm) ???
                        // gpx time 10748130358 2000599137 : garage-Dr_Smau => 15228.37m (16km osm)
                        // gpx distance 10748130358 2971599465 : garage-Denis_Rebours => 17313.70m (18km osm)
                        match shortest_path( input[1], &g.get_directed(), input[2].parse::<i64>().unwrap(), input[3].parse::<i64>().unwrap() ) {
                            Some(bt) => {
                                let mut data : Gpx = Default::default();
                                data.version = GpxVersion::Gpx11;

                                let mut trkseg: TrackSegment = TrackSegment::new();
                                let mut track: Track = Track::new();

                                let mut lat_min: f64 = 95.0;
                                let mut lat_max: f64 = -95.0;
                                let mut lon_min: f64 = 180.0;
                                let mut lon_max: f64 = -180.0;

                                for (_k, v) in bt.iter() {
                                    match &g.tnodes.get(&v) {
                                        Some(n) => {
                                            let lat = n.lat();
                                            let lon = n.lon();

                                            if lat < lat_min { lat_min = lat; }
                                            if lat > lat_max { lat_max = lat; }
                                            if lon < lon_min { lon_min = lon; }
                                            if lon > lon_max { lon_max = lon; }
                                            let pt = Waypoint::new( Point::new( lon, lat ) );
                                            trkseg.points.push( pt );
                                        },
                                        None => {
                                            println!("node id {} must be in db", v );
                                        },
                                    }
                                }
                                let mut meta: Metadata = Default::default();
                                let rect = Rect::new(
                                    coord! { x: lon_min, y: lat_min},
                                    coord! { x: lon_max, y: lat_max},
                                );
                                meta.bounds = Some( rect );
                                data.metadata = Some( meta );
                                let f = File::create("./data/trace.gpx").expect("Unable to create file");
                                track.segments.push( trkseg );
                                data.tracks.push( track );
                                write(&data, f).unwrap();
                            },
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
