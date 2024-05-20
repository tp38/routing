use std::fmt;

use serde::Deserialize;
use std::collections::HashMap;

use reqwest::{Error,header::HeaderValue};
use crate::graphe::elements::TNode;
use crate::routing::distances::distance_haversine;


#[derive(Deserialize, Debug)]
pub struct Location {
    pub place_id: i64,
    pub licence: String,
    pub osm_type: String,
    pub osm_id: i64,
    pub boundingbox: Vec<String>,
    pub lat: String,
    pub lon: String,
    pub display_name: String,
    pub class: String,
    pub r#type:String,
    pub importance: f64,
}

impl fmt::Display for Location {

    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!( f, "{} ({},{}): {}", self.osm_id, self.lat, self.lon, self.display_name )
    }

}

#[derive(Debug)]
pub struct Address {
    pub ad: String,
}

impl Address {

    pub fn get_url_string(&self) -> String {
        self.ad.replace( ' ', &"+" )
    }
}


pub fn get_location_from_nominatim(addr: &Address) -> Result<Vec<Location>, Error> {
    let hv = HeaderValue::from_str( "teepi_route").unwrap();
    let client = reqwest::blocking::Client::builder()
            .user_agent( hv )
            .build()
            .unwrap();
    let s: String = addr.get_url_string();
    let request_url = format!("https://nominatim.openstreetmap.org/search?q={}&format=json", s);
    let res = client.get(&request_url).send()?;
    let content = res.text()?;

    let vloc: Vec<Location> = serde_json::from_str( content.as_str() ).unwrap();
    Ok(vloc)
}


/// find the nearest node to the specified coordinates (lat, lon) and the associated distance
pub fn nearest_node( lat: f64, lon: f64, nodes: &HashMap<i64, TNode> ) -> (i64, f64) {
    let mut min_value:f64 = 100000000000.0;
    let mut id: i64 = 0;

    for (pid, p) in nodes {
        let dist = distance_haversine( lat, lon, p.lat(), p.lon() );
        if dist < min_value {
            min_value = dist;
            id = *pid;
        }
    }
    (id, min_value)
}

#[cfg(test)]
mod location_tests {
    use super::*;
    use crate::read_osm;

    #[test]
    fn test_nominatim() {
        let addr = vec!["22", "route", "des", "noels", "22800", "plaine-haute"];
        let ad: Address =  Address { ad: addr.join( " " ) };
        match get_location_from_nominatim( &ad ) {
            Ok(lv) => {
                assert_eq!( 1, lv.len() );
                assert_eq!( lv[0].osm_type, "way".to_string() );
                assert_eq!( lv[0].osm_id, 225806813 );
            },
            Err(_e) => {
                assert!( false );
            },
        }

    }

    #[test]
    fn test_nearest_node() {
        let g = read_osm( "/home/th/Code/Rust/route/data/routable.osm.pbf" );

        // nearest 48.44725 -2.86572 --> Pascal&Nathalie
        assert_eq!( (2345943396, 17.27768193285879), nearest_node( 48.44725, -2.86572, &g.tnodes ));
        // nearest 48.40631 -2.81467 --> garage
        assert_eq!( (10748130358, 31.36150979129478), nearest_node( 48.40631, -2.81467, &g.tnodes ));
        // nearest 48.40627 -2.81457 --> cuisine
        assert_eq!( (10748130360, 28.98867131727752), nearest_node( 48.40627, -2.81457, &g.tnodes ));
        // nearest 48.40672 -2.81433 --> Maryse
        assert_eq!( (4779385124, 19.205031153965326), nearest_node( 48.40672, -2.81433, &g.tnodes ));
        // nearest 48.41119 -2.81940 --> Pharmacie Plaintel
        assert_eq!( (7194631845, 27.349928257245583), nearest_node( 48.41119, -2.81940, &g.tnodes ));
        // nearest 48.34743 -2.75695 --> Parmacie Ploeuc
        assert_eq!( (10048845537, 9.669225837906378), nearest_node( 48.34743, -2.75695, &g.tnodes ));
    }

}
