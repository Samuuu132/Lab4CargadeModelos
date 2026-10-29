use std::fs;
use raylib::prelude::*;

pub struct Obj {
    pub vertices: Vec<Vector3>,
    pub indices: Vec<[usize; 3]>,
}

fn parse_index(token: &str, vertex_count: usize) -> usize {
    let value: i64 = token
        .split('/')
        .next()
        .and_then(|v| v.parse().ok())
        .expect("Indice de cara invalido");

    if value < 0 {
        (vertex_count as i64 + value) as usize
    } else {
        (value - 1) as usize
    }
}

impl Obj {
    pub fn load(path: &str) -> Obj {
        let content = fs::read_to_string(path).expect("No se pudo leer el archivo OBJ");

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for line in content.lines() {
            let mut parts = line.split_whitespace();

            match parts.next() {
                Some("v") => {
                    let coords: Vec<f32> = parts.take(3).map(|n| n.parse().unwrap()).collect();
                    vertices.push(Vector3::new(coords[0], coords[1], coords[2]));
                }
                Some("f") => {
                    let face: Vec<usize> = parts.map(|t| parse_index(t, vertices.len())).collect();

                    for i in 1..face.len() - 1 {
                        indices.push([face[0], face[i], face[i + 1]]);
                    }
                }
                _ => {}
            }
        }

        Obj { vertices, indices }
    }
}
