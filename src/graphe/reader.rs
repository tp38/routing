use super::*;

use osmpbf::{ElementReader, Element};
use crate::graphe::elements::{TNode, TWay, Graph};

fn register_tnode( dne: DenseNode, tnodes: &mut HashMap<i64, TNode> ) {
    let p = TNode::from( dne.clone() );
    tnodes.insert( dne.id(), p );
}

fn register_tway( we: Way, tways: &mut HashMap<i64, TWay>, tnodes: &HashMap<i64, TNode>  ) {
    let mut routable = false;
    let mut accessible = true;
    for (k, v) in we.tags() {
        // type de route
        if k == "highway" {
            match v {
                "motorway" | "trunk" | "primary" | "secondary" | "tertiary" | "residential"
                | "motorway_link" | "trunk_link" | "primary_link" | "secondary_link"
                | "tertiary_link" => { routable = true; },
                _ => {},
            }
        }
        // access
        if k == "access" {
            match v {
                "yes" => {},
                _ => { accessible = false; },
            }
        }
    }
    if routable & accessible {
        let b = TWay::from( we.clone(), tnodes );
        tways.insert( we.id(), b );
    }
}

pub fn read_osm(filename: &str ) -> Graph {
    let mut tnodes: HashMap<i64, TNode> = HashMap::new();
    let mut tways: HashMap<i64, TWay> = HashMap::new();

    let reader = ElementReader::from_path( filename ).unwrap();

    reader.for_each( |element| {
        match element {
            Element::DenseNode(dne) => { register_tnode( dne, &mut tnodes ); },
            Element::Way(we) => { register_tway( we, &mut tways, &tnodes ); },
            _ => {},
        }
    } ). unwrap();
    let mut g = Graph::new( filename.to_string(), tnodes, tways);
    g.init();
    g
}


#[cfg(test)]
mod reader_tests {
    use super::*;

    #[test]
    fn read_osmfile() {
        let g = read_osm( "/home/th/Code/Rust/route/data/routable.osm.pbf" );

        assert_eq!( 26154, g.tways.len() ); // 26154 calculé à partir des resultats osmium (cf data/osmium_cde.txt)
        assert_eq!( 221939, g.tnodes.len() ); // 221773 calculé à partir des resultats osmium (cf data/osmium_cde.txt)
    }

}
