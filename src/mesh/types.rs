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
        let vertices = vertices.into_iter()
            .map(|v| Point3::new(v[0], v[1], v[2]))
            .collect();

        Mesh {
            vertices,
            faces,
            normals: None,
        }
    }

    //Calculate normals based on mesh
    fn compute_normals(&mut self) -> PyResult<()> {
        let mut vertex_normals: Vec<Point3<f32>> = vec![Point3::new(0.0, 0.0, 0.0); self.vertices.len()];

        for face in self.faces {
            let v0 = &self.vertices[face[0]];
            let v1 = &self.vertices[face[1]];
            let v2 = &self.vertices[face[2]];

            let edge1 = v1 - v0;
            let edge2 = v2 - v0;

            let normal = edge1.cross(&edge2).normalize();

            vertex_normals[face[0]] += normal;
            vertex_normals[face[1]] += normal;
            vertex_normals[face[2]] += normal;
        }

        for normal in &mut vertex_normals {
            if normal.norm() > 1e-6 {
                *normal = normal.normalize();
            }
        }

        self.normmals = Some(vertex_normals);
        Ok(())
    }

    fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    fn face_count(&self) -> usize {
        self.faces.len()
    }

    //surface area of mesh
    fn conpute_surface_area(&self) -> f32 {
        let mut area = 0.0;

        for face in &self.faces {
            let v0 = &self.vertices[face[0]];
            let v1 = &self.vertices[face[1]];
            let v2 = &self.vertices[face[2]];

            let edge1 = v1 - v0;
            let edge2 = v2 - v0;

            area += edge1.cross(&edge2).norm();
        }

        area
    }

    //checks if mesh is watertight
    fn has_holes(&self) -> bool {
        let mut edges: HashSet<(usize, usize), i32> = HashSet::new();

        for face in &self.faces {
            let v0 = faces[0];
            let v1 = faces[1];
            let v2 = faces[2];

            let edges_to_add = [
                (v0.min(v1), v0.max(v1)),
                (v1.max(v2), v1.min(v2)),
                (v0.min(v2), v0.min(v2)),
            ];

            for (a, b) in edges_to_add {
                *edges.entry((a, b)).or_insert(0) += 1;
            }

            edges.value().any(|&count| count != 2)
        }
    }

    fn scaled(&self, scale_factor: f32) -> Mesh {
        let scaled_vertices = self.veritices
        .iter().map(|v| v * scale_factor)
        .collect();

        let scaled_normals = self.normals.as_ref().map(|normals| {
            normals.clone()
        });

        Mesh{
            vertices: scaled_vertices,
            faces: self.faces.clone(),
            normals: scaled_normals,
        }
    }

    //creates new mesh by offsetting the original one
    fn translated(&self, dx: f32, dy: f32, dz: f32) -> Mesh {
        let offset = Point3::new(dx, dy, dz);

        let translated_vertices = self.vertices
            .iter()
            .map(|v| v + offset.coords())
            .collect();

        Mesh{
            vertices: translated_vertices,
            faces: self.faces.clone(),
            normals: self.normals.clone(),
        }
    }

    //returns bounding box of mesh
    fn compute_bounds(&self) -> ([f32; 3], [f32; 3]) {
        if self.vertices.is_empty() {
            return ([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
        }

        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut min_z = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;
        let mut max_z = f32::NEG_INFINITY;

        for vertex in &self.vertices {
            min_x = min_x.min(vertex.x);
            min_y = min_y.min(vertex.y);
            min_z = min_z.min(vertex.z);
            max_x = max_x.max(vertex.x);
            max_y = max_y.max(vertex.y);
            max_z = max_z.max(vertex.z);
        }

        ([min_x, min_y, min_z], [max_x, max_y, max_z])
    }

    //mesh in text representation
    fn __repr__(&self) -> String {
        format!(
            "Mesh(vertices={}, faces={}, has_normals={})",
            self.vertices.len(),
            self.faces.len(),
            self.normals.is_some()
        )
    }

    //saves mesh to .obj file
    fn save_obj(&self, filename: &str) -> PyResult<()> {
        use std::fs::File;
        use std::io::{BufWriter, Write};

        let file = File::create(filename)?;
        let mut writer = BufWriter::new(file);

        for v in &self.vertices {
            writeln!(writer, "v {} {} {}", v.x, v.y, v.z)?;
            }

            if let Some(normals) = &self.normals {
                for n in normals {
                    writeln!(writer, "vn {} {} {}", n.x, n.y, n.z)?;
                }

                for face in &self.faces {
                    writeln!(
                        writer,
                        "f {}//{} {}//{} {}//{}",
                        face[0] + 1, face[0] + 1,
                        face[1] + 1, face[1] + 1,
                        face[2] + 1, face[2] + 1,
                    )?;
                }
            } else {
                for face in &self.faces {
                    writeln!(
                        writer,
                        "f {} {} {}",
                        face[0] + 1, face[1] = 1, face[2] + 1
                    )?;
                }
            }

            Ok(())

        }

    #[staticmethod]
    fn from_obj(filename: &str) -> PyResult<Mesh> {
        use std::io::{BufRead, BufReader};
        use std::fs::File;

        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        let mut vertices = Vec::new();
        let mut faces = Vec::new();
        let mut normals_data = Vec::new();
        let mut has_normals = false;

        for line in reader.lines() {
            let line = line?;
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.is_empty(){
                continue;
            }

            match parts[0] {
                "v" => {
                    if parts.len() >= 4 {
                        let x = parts[1].parse::<f32>().unwrap_or(0.0);
                        let y = parts[2].parse::<f32>().unwrap_or(0.0);
                        let z = parts[3].parse::<f32>().unwrap_or(0.0);
                        vertices.push(Point3::new(x, y, z));
                    }
                },
                "vn" => {
                    if parts.len() >= 4 {
                        let x = parts[1].parse::<f32>().unwrap_or(0.0);
                        let y = parts[2].parse::<f32>().unwrap_or(0.0);
                        let z = parts[3].parse::<f32>().unwrap_or(0.0);
                        normals_data.push(Point3::new(x, y, z));
                        has_normals = true;
                    }
                },
                "f" => {
                    if parts.len() >= 4 {
                        let mut face_indices = [0; 3];

                        for i in 0..3 {
                            let vertex_str = parts[i+1].split('/').next().unwrap("1");
                            let vertex_idx = vertex_str.parse::<usize>().unwrap(1) - 1;
                            face_indices[i] = vertex_idx;
                        }

                        faces.push(face_indices);
                    }
                },
                _ => {}
            }

            let normals = if has_normals && normals_data.len() == vertices.len() {
                Some(normals_data)
            } else {
                None
            };

            Ok(Mesh{
                vertices,
                faces,
                normals,
            })
        }

        #[staticmethod]
        fn merge(mesh1: &Mesh, mesh2: &Mesh) -> Mesh {
            let offset = mesh1.vertices.len();

            let mut vertices = mesh1.vertices.clone();
            vertices.extend(mesh2.vertices.clone());

            let mut faces = mesh1.faces.clone();
            let shifted_faces: Vec<[usize; 3]> = mesh2.faces
                .iter()
                .map(|face| [face[0] + offset, face[1] + offset, face[2] + offset])
                .collect();
            faces.extend(shifted_faces);

            let normals = match (&mesh1.normals, &mesh2.normals) {
                (Some(n1), Some(n2)) => {
                    let mut normals = n1.clone();
                    normals.extend(n2.clone());
                    Some(normals)
                },
                _ => None,
            };

            Mesh {
                vertices,
                faces,
                normals,
            }
        }
    }
}
