const EARTH_RADIUS: f64 = 6371.0;

#[no_mangle]
pub extern "C" fn haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();   

    let a = (dlat / 2.0).sin().powi(2) 
        + (dlon / 2.0).sin().powi(2) * lat1.to_radians().cos() * lat2.to_radians().cos();

    let c = 2.0 * a.sqrt().asin();

    let distance = c * EARTH_RADIUS;

    distance
}