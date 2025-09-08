/*
Purpose: Stores reusable geometry definitions
Responsibilities:
    - Constant arrays for simple shapes (TRIANGLE_VERTICES, SQUARE_VERTICES)
    - Functions like create_circle(radius, segments, color) for procedural geometry
    - ex: lego bricks
*/

use crate::vertex::Vertex;

pub const TRIANGLE_VERTICES: &[Vertex] = &[
    Vertex {position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0]}, // top
    Vertex {position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0]}, // bottom left
    Vertex {position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0]}, // bottom right
];

pub const SQUARE_VERTICES: &[Vertex] = &[
    Vertex {position: [-0.25, 0.25, 0.0], color: [1.0, 1.0, 0.0]}, // top left
    Vertex {position: [0.25, 0.25, 0.0], color: [0.0, 1.0, 1.0]}, // top right
    Vertex {position: [0.25, -0.25, 0.0], color: [1.0, 0.0, 1.0]}, // bottom right
    Vertex {position: [-0.25, -0.25, 0.0], color: [1.0, 0.5, 0.0]}, // bottom left
];

// Indices define which vertices make up triangles
pub const SQUARE_INDICES: &[u16] = &[
    0, 1, 2, // first triangle
    0, 2, 3, // second triangle
];



pub fn create_circle(radius: f32, segments: usize, color: [f32; 3]) -> (Vec<Vertex>, Vec<u16>) {
    // Imagine a pizza: one vertex at the center, then a ring of vertices around the edge
    // Each slice (center + two edge points) is one triangle
    // Put enough slices together -> looks like a circle
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Center vertex
    vertices.push(Vertex {
        position: [0.0, 0.0, 0.0],
        color,
    });

    // Create edge vertices around the circle
    for i in 0..=segments {
        let theta = (i as f32 / segments as f32) * std::f32::consts::TAU; // TAU = 2pi
        let x = radius * theta.cos();
        let y = radius * theta.sin();

        vertices.push(Vertex {
            position: [x,y, 0.0],
            color,
        });

        // Add indices to form triangles (skip first edge)
        if i > 0 {
            indices.push(0); // center
            indices.push(i as u16);
            indices.push((i as u16) + 1);
        }
    }

    (vertices, indices)
}