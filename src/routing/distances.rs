
///
/// calcul la distance entre 2 jeu de coordonnées lat, lon en utilisant pythagore
///
pub fn distance_pythagore( xa: f64, ya: f64, xb: f64, yb:f64 ) -> f64 {
    let x = ( xb - xa )*( ( ya + yb )/2.0 ).cos();
    let y = yb - ya;
    let z = ( x.powi(2) + y.powi(2) ).sqrt();
    let d = 1852.0 * 60.0 * z;
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


#[cfg(test)]
mod distance_tests {
    use super::*;


    #[test]
    fn test_pythagore() {
        assert_eq!( 111120.0 , distance_pythagore( 51.0, 2.0, 51.0, 3.0 ) );
        assert_eq!( 111120.0 , distance_pythagore( 51.0, 3.0, 51.0, 2.0 ) );
    }

    #[test]
    fn test_sinus() {
        assert_eq!( 111194.92664458815 , distance_sinus( 68.0, 51.0, 69.0, 51.0 ) );
        assert_eq!( 111194.92664458815 , distance_sinus( 69.0, 51.0, 68.0, 51.0 ) );
    }

    #[test]
    fn test_haversine() {
        assert_eq!( 111194.9266445603 , distance_haversine( 68.0, 51.0, 69.0, 51.0 ) );
        assert_eq!( 111194.9266445603 , distance_haversine( 69.0, 51.0, 68.0, 51.0 ) );
    }
}
