use crate::mesh::types::Mesh;
use nalgebra::Point3;
use pyo3::prelude::*;
use std::collections::{HashMap, HashSet};

pub struct TopologyAnalyzer {
    mesh: Mesh,
    edge_to_faces: HashMap<(usize, usize), Vec<usize>>,
    vertex_to_faces: HashMap<usize, Vec<usize>>,
}

impl TopologyAnalyzer {
    pub fn new(mesh: Mesh) -> Self{
        let mut analyzer = TopologyAnalyzer{
            mesh,
            edge_to_faces: HashMap::new(),
            vertex_to_faces: HashMap::new(),
        };

        analyzer.build_topology_maps();
        analyzer
    }

    fn build_topology_maps(&mut self) {
        for (face_idx, face) in self.mesh.faces.iter().enumerate() {

            for &vertex_idx in face {
                self.vertex_to_faces
                    .entry(vertex_idx)
                    .or_insert_with(Vec::new)
                    .push(face_idx);
            }

            let edges = [
                (face[0].min(face[1]), face[0].max(face[1])),
                (face[1].min(face[2]), face[1].max(face[2])),
                (face[2].min(face[0]), face[2].max(face[0])),
            ];

            for edge in edges{
                self.edge_to_faces
                    .entry(edge)
                    .or_insert_with(Vec::new)
                    .push(face_idx);
            }
        }
    }

    pub fn is_watertight(&self) -> bool {
        self.edge_to_faces
            .values()
            .all(|faces| faces.len() == 2 )
    }

    pub fn is_sphere_like(&self) -> bool {
        let v = self.mesh.vertices().len();
        let f = self.mesh.faces().len();
        let e = self.edge_to_faces.len();

        self.is_watertight() && v - e + f == 2
    }

    pub fn find_holes(&self) -> Vec<Vec<usize>> {
        let mut boundary_edges: Vec<(usize, usize)> = self.edge_to_faces
            .iter()
            .filter(|_, faces| faces.len() == 1)
            .map(|&edge, _| *edge)
            .collect();

        if boundary_edges.is_empty() {
            return Vec::new();
        }

        let mut holes = Vec::new();
        let mut remaining_edges = boundary_edges.clone();

        while !remaining_edges.is_empty() {
            let mut holes = Vec::new();
            let mut current_edge = remaining_edges.pop().unwrap();
            let mut current_vertex = current_edge.1;

            hole.push(current_vertex.0);
            hole.push(current_vertex);

            while let Some(pos) = remaining_edges.iter().position(|&(a,b)|{
                a == current_vertex || b == current_vertex
            }) {
                let edge = remaining_edges.swap_remove(pos);
                current_vertex = if egde.0 == current_vertex { edge.1 } else { edge.0 };
                hole.push(current_vertex);
            }

            holes.push(hole);

        }

        holes
    }
}

#[pyclass]
pub struct PyTopologyAnalyzer {
    analyzer: TopologyAnalyzer,
}

#[pymethods]
impl PyTopologyAnalyzer {
    #[new]
    fn new(mesh: &Mesh) -> Self {
        PyTopologyAnalyzer{
            analyzer: TopologyAnalyzer::new(mesh.clone()),
        }
    }

    fn is_watertight(&self) -> bool {
        self.analyzer.is_watertight()
    }

    fn is_sphere_like(&self) -> bool {
        self.analyzer.is_sphere_like()
    }

    fn find_holes(&self) -> Vec<Vec<usize>> {
        self.analyzer.find_holes()
    }
}