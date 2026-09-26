pub mod osm;
pub mod generator;
pub mod format;

pub use osm::parser::OSMDocument;
pub use generator::road_builder::build_roads;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pyfunction]
fn process_osm_to_blocks(osm_json_str: &str) -> PyResult<String> {
    Ok(format!("Processed OSM payload length: {}", osm_json_str.len()))
}

#[cfg(feature = "python")]
#[pymodule]
fn osm2mc_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(process_osm_to_blocks, m)?)?;
    Ok(())
}
