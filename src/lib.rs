use pyo3::prelude::*;
mod mesh;
mod analyzers;

use mesh::types::Mesh;
use analyzers::topology::PyTopologyAnalyzer;

#[pymodule]
fn meshalyzer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Mesh>()?;
    m.add_class::<PyTopologyAnalyzer>()?;
    Ok(())
}

#[pyfunction]
fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}