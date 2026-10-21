use raylib::prelude::{Vector3, Vector2};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Object {
    pub vertices: Vec<Vector3>,
    pub indices: Vec<u32>,
}


// Loads a 3D object from an .obj file and returns an Object struct containing the vertices and indices.
pub fn load_obj(path: &str) -> Result<Object, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut vertices: Vec<Vector3> = Vec::with_capacity(1024);
    let mut indices: Vec<u32> = Vec::with_capacity(2048);
    let mut line_buffer = String::with_capacity(128);

    while reader.read_line(&mut line_buffer)? > 0 {
        // Trim the line to remove whitespace and check if it's empty or a comment
        let trimmed = line_buffer.trim();
        
        // Skip empty lines and comments
        if trimmed.is_empty() || trimmed.starts_with('#') {
            line_buffer.clear();
            continue;
        }

        // Split the line into parts and process based on the prefix
        let mut parts = trimmed.split_whitespace();
        if let Some(prefix) = parts.next() {
            match prefix {
                "v" => {
                    let x: f32 = parts.next().ok_or("Missing x")?.parse()?;
                    let y: f32 = parts.next().ok_or("Missing y")?.parse()?;
                    let z: f32 = parts.next().ok_or("Missing z")?.parse()?;
                    vertices.push(Vector3::new(x, y, z));
                }
                "f" => {
                    // Obtain the vertex indices for the face
                    let mut face_indices: Vec<u32> = Vec::new();
                    for token in parts {
                        // Split the token by '/' to handle cases like "1/2/3" and take the first part as the vertex index
                        let v_str = token.split('/').next().ok_or("Invalid face format")?;
                        let v_idx: usize = v_str.parse()?;
                        
                        // Convert to 0-based index and push to face_indices
                        face_indices.push((v_idx - 1) as u32);
                    }

                    // Triangulate the face if it has more than 3 vertices (fan triangulation)
                    if face_indices.len() >= 3 {
                        for i in 1..(face_indices.len() - 1) {
                            indices.push(face_indices[0]);
                            indices.push(face_indices[i]);
                            indices.push(face_indices[i + 1]);
                        }
                    }
                }
                _ => {}
            }
        }

        line_buffer.clear();
    }

    Ok(Object { vertices, indices })
}



// Projects a 3D vertex to 2D screen space using perspective projection.
pub fn project_vertex(
    v: Vector3,
    camera_pos: Vector3,
    fov_scale: f32,
    width: f32,
    height: f32,
) -> Option<Vector2> {
    // Calculate the depth (z-coordinate) relative to the camera position
    let z = v.z - camera_pos.z;

    if z <= 0.1 {
        return None;
    }

    // Apply perspective projection formula to convert 3D coordinates to 2D screen space
    let x = (v.x - camera_pos.x) / z * fov_scale + (width / 2.0);
    let y = -(v.y - camera_pos.y) / z * fov_scale + (height / 2.0);

    Some(Vector2::new(x, y))
}
