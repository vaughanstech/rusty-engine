/*
Purpose: Stores reusable geometry definitions
Responsibilities:
    - Constant arrays for simple shapes (TRIANGLE_VERTICES, SQUARE_VERTICES)
    - Functions like create_circle(radius, segments, color) for procedural geometry
    - ex: lego bricks
*/

use crate::vertex::Vertex;

pub fn create_plane() -> (Vec<Vertex>, Vec<u16>) {
    let mut plane_vertices = vec![
        // Bottom Left
        Vertex { position: [-5.0, 0.0, -5.0], normal: [0.0, 0.0, 0.0], tex_coords: [0.0, 0.0], color: [0.3, 0.3, 0.3] },
        // Bottom Right
        Vertex { position: [5.0, 0.0, -5.0], normal: [0.0, 0.0, 0.0], tex_coords: [1.0, 0.0], color: [0.3, 0.3, 0.3] },
        // Top Right
        Vertex { position: [5.0, 0.0, 5.0], normal: [0.0, 0.0, 0.0], tex_coords: [1.0, 1.0], color: [0.3, 0.3, 0.3] },
        // Top Left
        Vertex { position: [-5.0, 0.0, 5.0], normal: [0.0, 0.0, 0.0], tex_coords: [0.0, 1.0], color: [0.3, 0.3, 0.3] },
    ];

    let plane_indices = vec![
        0, 1, 2, // first triangle
        0, 2, 3, // second triangle
    ];

    Vertex::compute_normals(&mut plane_vertices, &plane_indices);

    (plane_vertices, plane_indices)
}


pub fn create_pyramid() -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = vec![
        // Base (y = 0, facing downward - normal = (0, -1, 0))
        Vertex { position: [-0.5, 0.0, -0.5], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 0.0], color: [1.0, 0.0, 0.0] },
        Vertex { position: [ 0.5, 0.0, -0.5], normal: [0.0, -1.0, 0.0], tex_coords: [1.0, 0.0], color: [0.0, 1.0, 0.0] },
        Vertex { position: [ 0.5, 0.0,  0.5], normal: [0.0, -1.0, 0.0], tex_coords: [1.0, 1.0], color: [0.0, 0.0, 1.0] },
        Vertex { position: [-0.5, 0.0,  0.5], normal: [0.0, -1.0, 0.0], tex_coords: [0.0, 1.0], color: [1.0, 1.0, 0.0] },

        // Front face (apex + front base edge) -> normal points forward
        Vertex { position: [0.0, 1.0, 0.0], normal: [0.0, 0.5, -0.866], tex_coords: [0.5, 1.0], color: [1.0, 1.0, 1.0] }, // apex
        Vertex { position: [-0.5, 0.0, -0.5], normal: [0.0, 0.5, -0.866], tex_coords: [0.0, 0.0], color: [1.0, 0.0, 0.0] },
        Vertex { position: [ 0.5, 0.0, -0.5], normal: [0.0, 0.5, -0.866], tex_coords: [1.0, 0.0], color: [0.0, 1.0, 0.0] },

        // Right face
        Vertex { position: [0.0, 1.0, 0.0], normal: [0.866, 0.5, 0.0], tex_coords: [0.5, 1.0], color: [1.0, 1.0, 1.0] },
        Vertex { position: [0.5, 0.0, -0.5], normal: [0.866, 0.5, 0.0], tex_coords: [1.0, 0.0], color: [0.0, 1.0, 0.0] },
        Vertex { position: [0.5, 0.0,  0.5], normal: [0.866, 0.5, 0.0], tex_coords: [1.0, 1.0], color: [0.0, 0.0, 1.0] },

        // Back face
        Vertex { position: [0.0, 1.0, 0.0], normal: [0.0, 0.5, 0.866], tex_coords: [0.5, 1.0], color: [1.0, 1.0, 1.0] },
        Vertex { position: [0.5, 0.0, 0.5], normal: [0.0, 0.5, 0.866], tex_coords: [1.0, 1.0], color: [0.0, 0.0, 1.0] },
        Vertex { position: [-0.5, 0.0, 0.5], normal: [0.0, 0.5, 0.866], tex_coords: [0.0, 1.0], color: [1.0, 1.0, 0.0] },

        // Left face
        Vertex { position: [0.0, 1.0, 0.0], normal: [-0.866, 0.5, 0.0], tex_coords: [0.5, 1.0], color: [1.0, 1.0, 1.0] },
        Vertex { position: [-0.5, 0.0, 0.5], normal: [-0.866, 0.5, 0.0], tex_coords: [0.0, 1.0], color: [1.0, 1.0, 0.0] },
        Vertex { position: [-0.5, 0.0, -0.5], normal: [-0.866, 0.5, 0.0], tex_coords: [0.0, 0.0], color: [1.0, 0.0, 0.0] },
    ];

    let indices: Vec<u16> = vec![
        // Base
        0, 1, 2,
        0, 2, 3,

        // Sides
        4, 5, 6,   // front
        7, 8, 9,   // right
        10, 11, 12, // back
        13, 14, 15, // left
    ];

    Vertex::compute_normals(&mut vertices, &indices);

    (vertices, indices)
}

pub fn create_cube() -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = vec![
        // Front face (+Z)
        Vertex { position: [-0.5, -0.5,  0.5], color: [1.0, 0.0, 0.0], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, 1.0] },
        Vertex { position: [ 0.5, -0.5,  0.5], color: [0.0, 1.0, 0.0], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, 1.0] },
        Vertex { position: [ 0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, 1.0] },
        Vertex { position: [-0.5,  0.5,  0.5], color: [1.0, 1.0, 0.0], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, 1.0] },

        // Back face (-Z)
        Vertex { position: [-0.5, -0.5, -0.5], color: [1.0, 0.0, 1.0], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, -1.0] },
        Vertex { position: [ 0.5, -0.5, -0.5], color: [0.0, 1.0, 1.0], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, -1.0] },
        Vertex { position: [ 0.5,  0.5, -0.5], color: [0.5, 0.5, 0.5], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, -1.0] },
        Vertex { position: [-0.5,  0.5, -0.5], color: [1.0, 0.5, 0.0], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, -1.0] },

        // Left face (-X)
        Vertex { position: [-0.5, -0.5, -0.5], color: [1.0, 0.0, 0.0], tex_coords: [0.0, 0.0], normal: [-1.0, 0.0, 0.0] },
        Vertex { position: [-0.5, -0.5,  0.5], color: [0.0, 1.0, 0.0], tex_coords: [1.0, 0.0], normal: [-1.0, 0.0, 0.0] },
        Vertex { position: [-0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0], tex_coords: [1.0, 1.0], normal: [-1.0, 0.0, 0.0] },
        Vertex { position: [-0.5,  0.5, -0.5], color: [1.0, 1.0, 0.0], tex_coords: [0.0, 1.0], normal: [-1.0, 0.0, 0.0] },

        // Right face (+X)
        Vertex { position: [ 0.5, -0.5, -0.5], color: [1.0, 0.0, 1.0], tex_coords: [1.0, 0.0], normal: [1.0, 0.0, 0.0] },
        Vertex { position: [ 0.5, -0.5,  0.5], color: [0.0, 1.0, 1.0], tex_coords: [0.0, 0.0], normal: [1.0, 0.0, 0.0] },
        Vertex { position: [ 0.5,  0.5,  0.5], color: [0.5, 0.5, 0.5], tex_coords: [0.0, 1.0], normal: [1.0, 0.0, 0.0] },
        Vertex { position: [ 0.5,  0.5, -0.5], color: [1.0, 0.5, 0.0], tex_coords: [1.0, 1.0], normal: [1.0, 0.0, 0.0] },

        // Top face (+Y)
        Vertex { position: [-0.5,  0.5, -0.5], color: [0.0, 1.0, 0.0], tex_coords: [0.0, 0.0], normal: [0.0, 1.0, 0.0] },
        Vertex { position: [-0.5,  0.5,  0.5], color: [0.0, 0.0, 1.0], tex_coords: [1.0, 0.0], normal: [0.0, 1.0, 0.0] },
        Vertex { position: [ 0.5,  0.5,  0.5], color: [1.0, 0.0, 0.0], tex_coords: [1.0, 1.0], normal: [0.0, 1.0, 0.0] },
        Vertex { position: [ 0.5,  0.5, -0.5], color: [1.0, 1.0, 0.0], tex_coords: [0.0, 1.0], normal: [0.0, 1.0, 0.0] },

        // Bottom face (-Y)
        Vertex { position: [-0.5, -0.5, -0.5], color: [0.0, 1.0, 1.0], tex_coords: [1.0, 0.0], normal: [0.0, -1.0, 0.0] },
        Vertex { position: [-0.5, -0.5,  0.5], color: [1.0, 0.0, 1.0], tex_coords: [0.0, 0.0], normal: [0.0, -1.0, 0.0] },
        Vertex { position: [ 0.5, -0.5,  0.5], color: [1.0, 0.5, 0.0], tex_coords: [0.0, 1.0], normal: [0.0, -1.0, 0.0] },
        Vertex { position: [ 0.5, -0.5, -0.5], color: [0.5, 0.5, 0.5], tex_coords: [1.0, 1.0], normal: [0.0, -1.0, 0.0] },
    ];

    let indices = vec![
        0, 1, 2, 0, 2, 3,    // front
        4, 5, 6, 4, 6, 7,    // back
        8, 9, 10, 8, 10, 11, // left
        12, 13, 14, 12, 14, 15, // right
        16, 17, 18, 16, 18, 19, // top
        20, 21, 22, 20, 22, 23, // bottom
    ];

    Vertex::compute_normals(&mut vertices, &indices);

    (vertices, indices)
}

pub fn create_sphere(radius: f32, sectors: u32, stacks: u32) -> (Vec<Vertex>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // vertices
    for i in 0..=stacks {
        let stack_angle = std::f32::consts::PI / 2.0 - i as f32 * std::f32::consts::PI / stacks as f32; // from pi/2 to -pi/2
        let xy = radius * stack_angle.cos();
        let z = radius *stack_angle.sin();

        for j in 0..=sectors {
            let sector_angle = j as f32 * 2.0 * std::f32::consts::PI / sectors as f32; // 0 to 2pi

            let x = xy * sector_angle.cos();
            let y = xy * sector_angle.sin();

            let nx = x / radius;
            let ny = y / radius;
            let nz = z / radius;

            let u = j as f32 / sectors as f32;
            let v = i as f32 / stacks as f32;

            vertices.push(Vertex {
                position: [x, y, z],
                color: [0.5, 0.5, 0.5], // default white
                tex_coords: [u, v],
                normal: [nx, ny, nz],
            });
        }
    }

    // indices
    for i in 0..stacks {
        let k1 = i * (sectors + 1);
        let k2 = k1 + sectors + 1;

        for j in 0..sectors {
            if i != 0 {
                indices.push((k1 + j) as u16);
                indices.push((k2 + j) as u16);
                indices.push((k1 + j + 1) as u16);
            }

            if i != (stacks - 1) {
                indices.push((k1 + j + 1) as u16);
                indices.push((k2 + j) as u16);
                indices.push((k2 + j + 1) as u16);
            }
        }
    }

    (vertices, indices)
}


// pub fn create_circle(radius: f32, segments: usize, color: [f32; 3], tex_coords: [f32; 2]) -> (Vec<Vertex>, Vec<u16>) {
//     // Imagine a pizza: one vertex at the center, then a ring of vertices around the edge
//     // Each slice (center + two edge points) is one triangle
//     // Put enough slices together -> looks like a circle
//     let mut vertices = Vec::new();
//     let mut indices = Vec::new();

//     // Center vertex
//     vertices.push(Vertex {
//         position: [0.0, 0.0, 0.0],
//         color,
//         tex_coords,
//     });

//     // Create edge vertices around the circle
//     for i in 0..=segments {
//         let theta = (i as f32 / segments as f32) * std::f32::consts::TAU; // TAU = 2pi
//         let x = radius * theta.cos();
//         let y = radius * theta.sin();

//         vertices.push(Vertex {
//             position: [x,y, 0.0],
//             color,
//             tex_coords,
//         });

//         // Add indices to form triangles (skip first edge)
//         if i > 0 {
//             indices.push(0); // center
//             indices.push(i as u16);
//             indices.push((i as u16) + 1);
//         }
//     }

//     (vertices, indices)
// }