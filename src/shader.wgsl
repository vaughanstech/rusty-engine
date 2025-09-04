// Vertex shader: takes position and passes it along
@vertex
fn vs_main(
    @location(0) in_pos: vec2<f32>
) -> @builtin(position) vec4<f32> {
    // Convert 2D pos -> 4D clip space position
    return vec4<f32>(in_pos, 0.0, 1.0);
}

// Fragment shader: outputs a fixed color
@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0); // red
}