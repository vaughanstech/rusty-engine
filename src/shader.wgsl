// Vertex shader: outputs position + color
struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) color: vec3<f32>,
};

// Vertex shader: takes position and passes it along
@vertex
fn vs_main(
    @location(0) in_pos: vec2<f32>,
    @location(1) in_color: vec3<f32>
) -> VertexOutput {
    var out: VertexOutput;
    out.pos = vec4<f32>(in_pos, 0.0, 1.0);
    out.color = in_color;
    return out;
}

// Fragment shader: takes color from vertex shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}