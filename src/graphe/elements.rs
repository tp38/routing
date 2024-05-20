use super::*;
use crate::routing::dijkstra::Edge;

use std::fmt;
use ansi_term::Colour;

///
/// calcul la distance entre 2 jeu de coordonnées lat, lon en utilisant pythagore
///
pub fn distance_pythagore( xa: f64, ya: f64, xb: f64, yb:f64 ) -> f64 {
    let x = ( xb - xa )*( ( ya + yb )/2.0 ).cos();
    let y = yb - ya;
    let z = ( x.sqrt() + y.sqrt() ).powf(0.5);
    let d = 1.852 * 60.0 * z;
    d
}

///
/// calcul la distance entre 2 jeu de coordonnées lat, lon suivant la loi des sinus
///
pub fn distance_sinus( xd: f64, yd: f64, zd: f64, td:f64 ) -> f64 {
    use core::f64::consts::PI;

    // conversion degrés -> radians
    let x = xd / 180.0 * PI;
    let y = yd / 180.0 * PI;
    let z = zd / 180.0 * PI;
    let t = td / 180.0 * PI;

    let d: f64 = 1000.0 * 6371.0*(z.sin()*x.sin() + z.cos()*x.cos()*(y-t).cos() ).acos();

    d
}

///
/// calcul la distance entre 2 jeu de coordonnées lat, lon suivant la formule de haversine
///
pub fn distance_haversine( xd: f64, yd: f64, zd: f64, td:f64 ) -> f64 {
    use core::f64::consts::PI;

    // conversion degrés -> radians
    let x = xd / 180.0 * PI;
    let y = yd / 180.0 * PI;
    let z = zd / 180.0 * PI;
    let t = td / 180.0 * PI;

    // formule de haversine
    let a: f64 = ((x-z)/2.0).sin().powi(2) + z.cos()*x.cos()*((y-t)/2.0).sin().powi(2);
    let c: f64 = 2.0*(a.sqrt()/(1.0-a).sqrt()).atan();
    let d: f64 = 1000.0 * 6371.0 * c;
    d
}


#[derive(Debug,Clone,Copy,PartialEq)]
pub enum TNodeType {
    UnUsed,
    EndNode,
    MiddleNode,
}


#[derive(Debug, Clone)]
pub struct TNode {
    #[doc(hidden)]
    lat: f64,
    #[doc(hidden)]
    lon: f64,
    #[doc(hidden)]
    tags: HashMap<String, String>,
    #[doc(hidden)]
    r#type: TNodeType,
}

///
/// representation d'un sommet dans le graphe
///
impl TNode {
    ///
    /// initialisation d'un nouvel objet à partir des valeurs fournies
    ///
    pub fn new( lat: f64, lon: f64,  t: HashMap<String,String> ) -> Self {
        let mut tags = HashMap::new();
        for (k, v) in t.iter() { tags.insert( k.to_string(), v.to_string() ); }
        Self { lat: lat, lon: lon, tags: tags, r#type: TNodeType::UnUsed }
    }

    ///
    /// création à partir d'un DenseNode (cf.osmpbf)
    ///
    pub fn from( dn: DenseNode ) -> Self {
        let mut tags = HashMap::new();
        for (k, v) in dn.tags() { tags.insert( k.to_string(), v.to_string() ); }
        Self { lat: dn.lat(), lon: dn.lon(), tags: tags, r#type: TNodeType::UnUsed  }
    }

    ///
    /// accès à la latitude
    ///
    pub fn lat(&self) -> f64 {
        self.lat
    }

    ///
    /// accès à la longitude
    ///
    pub fn lon(&self) -> f64 {
        self.lon
    }

    ///
    /// accès aux tags du point
    ///
    pub fn tags(&self) -> &HashMap<String, String> {
        &self.tags
    }

    ///
    /// accès au contenu d'un tag
    ///
    pub fn get(&self, s: &String ) -> Option<&String> {
        self.tags.get( s )
    }

    ///
    /// accès au TNodeType
    ///
    pub fn get_type(&self) -> TNodeType {
        self.r#type
    }

    ///
    /// modification du type
    ///
    pub fn set_type(&mut self, n: TNodeType ) -> TNodeType {
        self.r#type = n;
        self.r#type
    }
}

impl fmt::Display for TNode {
    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!( f, "\tcoords : ({} , {}) :",
            Colour::Green.paint( self.lat.to_string() ),
            Colour::Green.paint( self.lon.to_string() ) ).unwrap();
        if self.tags.len() > 0 {
            write!(f, "\n\ttags :" ).unwrap();
            for (k, v) in self.tags() {
                write!( f, "\t{} => {}\n", k, v).unwrap();
            }
        }
        write!(f, "")
    }
}


#[cfg(test)]
mod tnode_tests {
    use super::*;
    use osmpbf::{ElementReader,Element};


    #[test]
    fn create_new_tnode() {
        let mut tags = HashMap::new();
        tags.insert( "highway".to_string(), "trunk".to_string() );
        tags.insert( "oneway".to_string(), "yes".to_string() );
        tags.insert( "destination".to_string(), "Plaintel".to_string() );
        let mut p: TNode = TNode::new( 48.615564, -2.8260458, tags );

        assert_eq!( 48.615564, p.lat() );
        assert_eq!( -2.8260458, p.lon() );
        assert_eq!( Some(&"trunk".to_string()), p.get( &"highway".to_string() ) );
        assert_eq!( None, p.get( &"lanes".to_string() ) );
        assert_eq!( TNodeType::UnUsed, p.get_type() );
        assert_eq!( TNodeType::MiddleNode, p.set_type( TNodeType::MiddleNode ) );
    }

    #[test]
    fn tnode_from_densenode() {
        let reader = ElementReader::from_path( "/home/th/Code/Rust/route/data/Bretagne.osm.pbf" ).unwrap();
        let mut p: Option<TNode> = None;

        reader.for_each( |element| {
            if let Element::DenseNode(dne) = element {
                if dne.id() == 28994912 {
                    p = Some( TNode::from(dne) );
                }
            }
        } ). unwrap();
        if let Some(mut n) = p {
            assert_eq!( 48.534449, n.lat() );
            assert_eq!( -2.0139565, n.lon() );
            assert_eq!( Some(&"motorway_junction".to_string()), n.get( &"highway".to_string() ) );
            assert_eq!( None, n.get( &"lanes".to_string() ) );
            assert_eq!( TNodeType::UnUsed, n.get_type() );
            assert_eq!( TNodeType::EndNode, n.set_type( TNodeType::EndNode ) );
        }
    }

}


#[derive(Debug, Clone)]
pub struct TWay {
    refs: Vec<i64>,
    #[doc(hidden)]
    tags: HashMap<String, String>,
    #[doc(hidden)]
    len: f64,
}

impl TWay {
    ///
    /// initialisation à partir de valeurs données
    ///
    pub fn new( r: Vec<i64>, t: HashMap<String, String>, nodes: &HashMap<i64, TNode> ) -> Self {
        let mut refs: Vec<i64> = Vec::new();
        for i in r.iter() { refs.push( *i ); }
        let mut tags = HashMap::new();
        for (k, v) in t.iter() { tags.insert( k.to_string(), v.to_string()); }
        let Some(mut vo) = nodes.get( &refs[0] ) else { panic!( "start node must exist in db ... ")};
        let mut d = 0.0;
        for idx in refs.iter() {
            let Some(vc) = nodes.get( &idx ) else { panic!( "cur node must exist in db ... ")};
            d += distance_haversine( vo.lat(), vo.lon(), vc.lat(), vc.lon() );
            vo = vc
        }
        Self { refs: refs, tags: tags, len: d }
    }

    ///
    /// création à partir d'une way (cf. osmpbf) et de l'ensemble des Peaks
    ///
    pub fn from( we: Way, nodes: &HashMap<i64, TNode> ) -> Self {
        let mut refs: Vec<i64> = Vec::new();
        for i in we.refs() { refs.push( i ); }
        let mut tags = HashMap::new();
        for (k, v) in we.tags() { tags.insert( k.to_string(), v.to_string()); }
        let Some(mut vo) = nodes.get( &refs[0] ) else { panic!( "start node must exist in db ... ")};
        let mut d = 0.0;
        for idx in refs.iter() {
            let Some(vc) = nodes.get( &idx ) else { panic!( "cur node must exist in db ... ")};
            d += distance_haversine( vo.lat(), vo.lon(), vc.lat(), vc.lon() );
            vo = vc
        }
        Self { refs: refs, tags: tags, len: d }
    }

    ///
    /// accès aux références des noeuds/sommets
    ///
    pub fn refs(&self) -> &Vec<i64> {
        &self.refs
    }

    ///
    /// accès aux tags
    ///
    pub fn tags(&self) -> &HashMap<String, String> {
        &self.tags
    }

    ///
    /// le départ de l'Arc
    ///
    pub fn start(&self) -> i64 {
        self.refs[0]
    }

    ///
    /// l'arrivée de l'Arc
    ///
    pub fn end(&self) -> i64 {
        let Some(last) = self.refs.last() else { panic!("a Bow must have 2 Peak") };
        *last
    }

    ///
    /// accès à la longeur de l'Arc
    ///
    pub fn len(&self) -> f64 {
        self.len
    }

    ///
    /// voie en sens unique
    ///
    pub fn oneway(&self) -> bool {
        for (k, v) in self.tags.iter() {
            if k == "oneway" && v == "yes" { return true; }
        }
        false
    }
}


impl fmt::Display for TWay {
    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!( f, "\tnodes :" ).unwrap();
        for n in self.refs() {
            write!( f, " {},", Colour::Blue.paint( n.to_string() ) ).unwrap();
        }
        write!(f, "\n\tlongueur : {}", self.len).unwrap();
        if self.tags.len() > 0 {
            write!( f, "\n\ttags :\n" ).unwrap();
            for (k, v) in self.tags() {
                write!( f, "\t\t{} => {}\n", k, v).unwrap();
            }
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tway_tests {
    use super::*;
    use osmpbf::{ElementReader,Element};


    #[test]
    fn create_new_tway() {
        let reader = ElementReader::from_path( "/home/th/Code/Rust/route/data/Bretagne.osm.pbf" ).unwrap();
        let mut tnodes: HashMap<i64, TNode> = HashMap::new();

        reader.for_each( |element| {
            match element {
                Element::DenseNode(dne) => {
                    let p = TNode::from( dne.clone() );
                    tnodes.insert( dne.id(), p );
                },
                _ => {},
            }
        } ). unwrap();

        let refs: Vec<i64> = Vec::from( [28994912, 2974951021, 2974951022, 1074940318, 2974951020, 2974951019] );
        let mut tags = HashMap::new();
        tags.insert( "highway".to_string(), "trunk_link".to_string() );
        tags.insert( "oneway".to_string(), "yes".to_string() );
        tags.insert( "destination:ref".to_string(), "D 12".to_string() );
        let b = TWay::new( refs, tags, &tnodes);

        assert_eq!( 6, b.refs().len() );
        assert_eq!( 28994912, b.refs()[0] );
        assert_eq!( Some(&"trunk_link".to_string()), b.tags().get( &"highway".to_string() ) );
        assert_eq!( None, b.tags().get( &"lanes".to_string() ) );
        assert_eq!( 160.5895459714768, b.len() );
    }

    #[test]
    fn tway_from_way() {
        let reader = ElementReader::from_path( "/home/th/Code/Rust/route/data/Bretagne.osm.pbf" ).unwrap();
        let mut peaks = HashMap::new();
        let mut p: Option<TWay> = None;

        reader.for_each( |element| {
            match element {
                Element::DenseNode(dne) => {
                    let p = TNode::from( dne.clone() );
                    peaks.insert( dne.id(), p );
                },
                Element::Way(we) => {
                    if we.id() == 4945346 {
                        p = Some( TWay::from(we.clone(), &peaks) );
                    }
                },
                _ => {},
            }
        } ). unwrap();
        if let Some(n) = p {
            assert_eq!( 6, n.refs().len() );
            assert_eq!( 28994912, n.refs()[0] );
            assert_eq!( Some(&"trunk_link".to_string()), n.tags().get( &"highway".to_string() ) );
            assert_eq!( None, n.tags().get( &"road".to_string() ) );
            assert_eq!( 160.5895459714768, n.len() );
        }
    }

}

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
    pub fn init(&mut self) {
        for ( _i, e ) in self.tways.iter() {
            let mut count: usize = 0;
            let mut lastid: &i64 = &0;
            for n in e.refs.iter() {
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

        for (_k, w) in self.tways.iter() {
            // pour chacun des segments composant la voie (way)
            for i in 1..w.refs.len() {
                // les id des noeuds
                let start_idx = w.refs[i-1];
                let end_idx = w.refs[i];

                // les datas associées
                let start = self.tnodes.get( &start_idx ).expect( "start node must exist in db ... ");
                let end = self.tnodes.get( &end_idx ).expect( "end node must exist in db ... ");

                // calcul de la distance entre les noeuds
                // let d = distance_pythagore(  start.lat(), start.lon(), end.lat(), end.lon() );
                let d = distance_sinus( start.lat(), start.lon(), end.lat(), end.lon() );
                // let d = distance_haversine( start.lat(), start.lon(), end.lat(), end.lon() );

                // on crée un arc vers le nodeid de fin et comprenant la distance calculée
                let normal = Edge{ end_node: end_idx, cost: d };
                match graph.get_mut( &start_idx ) {
                    // l'entrée existe : on reajoute à la liste des arcs du noeud considéré
                    Some(v) => { v.push( normal ); },
                    // l'entrée n'existe pas : on ajoute le node avec une nouvelle liste
                    None => { graph.insert( start_idx, vec![ normal ] ); },
                };
                // on traite les voies a double sens en enregistrant l'arc contraire
                if ! w.oneway() {
                    let reverse = Edge{ end_node: start_idx, cost: d };
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
