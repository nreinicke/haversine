use cpython::{PyResult, Python, py_module_initializer, py_fn};

py_module_initializer!(libhaversine, initlibhaversine, PyInit_haversine, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "haversine", py_fn!(py, haversine_py(lat1: f64, lon1: f64, lat2: f64, lon2: f64)))?;
    Ok(())
});


const EARTH_RADIUS: f64 = 6371.0;

fn haversine(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    let dlat = (lat2 - lat1).to_radians();
    let dlon = (lon2 - lon1).to_radians();   

    let a = (dlat / 2.0).sin().powi(2) 
        + (dlon / 2.0).sin().powi(2) * lat1.to_radians().cos() * lat2.to_radians().cos();

    let c = 2.0 * a.sqrt().asin();

    let distance = c * EARTH_RADIUS;

    distance
}

fn haversine_py(_: Python, lat1:f64, lon1:f64, lat2:f64, lon2:f64) -> PyResult<f64> {
    let result = haversine(lat1, lon1, lat2, lon2);
    Ok(result)
}
