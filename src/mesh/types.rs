use nalgebra::Point3;
use pyo3::prelude::*;

#[pyclass]
pub struct Mesh {
    #[pyo3(get)]
    pub vertices: Vec<Point3<f32>>,
    #[pyo3(get)]
    pub faces: Vec<[usize; 3]>,
    #[pyo3(get)]
    pub normals: Option<Vec<Point3<f32>>>,
}

#[pymethods]
impl Mesh {
    #[new]
    fn new(vertices: Vec<[f32; 3]>, faces: Vec<[usize; 3]>) -> Self {
        //TODO: implement
    }


}