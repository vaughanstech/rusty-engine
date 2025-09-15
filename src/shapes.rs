/*
Purpose: Stores reusable geometry definitions
Responsibilities:
    - Constant arrays for simple shapes (TRIANGLE_VERTICES, SQUARE_VERTICES)
    - Functions like create_circle(radius, segments, color) for procedural geometry
    - ex: lego bricks
*/

use crate::vertex::Vertex;

pub const TRIANGLE_VERTICES: &[Vertex] = &[
    Vertex {position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0], tex_coords: [0.5, 0.0], normal: [0.0, 0.0, 0.0]}, // top
    Vertex {position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, 0.0]}, // bottom left
    Vertex {position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, 0.0]}, // bottom right
];
pub const TRIANGLE_INDICES: &[u16] = &[0, 1, 2];

pub const SQUARE_VERTICES: &[Vertex] = &[
    Vertex {position: [-0.25, 0.25, 0.0], color: [1.0, 1.0, 0.0], tex_coords: [0.0, 1.0], normal: [0.0, 0.0, 0.0]}, // top left
    Vertex {position: [0.25, 0.25, 0.0], color: [0.0, 1.0, 1.0], tex_coords: [1.0, 1.0], normal: [0.0, 0.0, 0.0]}, // top right
    Vertex {position: [0.25, -0.25, 0.0], color: [1.0, 0.0, 1.0], tex_coords: [1.0, 0.0], normal: [0.0, 0.0, 0.0]}, // bottom right
    Vertex {position: [-0.25, -0.25, 0.0], color: [1.0, 0.5, 0.0], tex_coords: [0.0, 0.0], normal: [0.0, 0.0, 0.0]}, // bottom left
];

// Indices define which vertices make up triangles
pub const SQUARE_INDICES: &[u16] = &[
    0, 1, 2, // first triangle
    0, 2, 3, // second triangle
];


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