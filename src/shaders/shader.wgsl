struct VertexPayload {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> VertexPayload {
    // Invoke for each of the corners of a shape
    // --> Write data that can be interpreted on a per/pixel level

    // Create position coordinates for each point of the shape
    // --> -x is to the left, +x is to the right
    // --> -y bottom of screen, +y top of screen
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-0.75, -0.75),
        vec2<f32>( 0.75,  0.75),
        vec2<f32>(  0.0,  0.75),
    );

    var colors = array<vec3<f32>, 3>(
        vec3<f32>(1.0, 0.0, 0.0),
        vec3<f32>(0.0, 1.0, 0.0),
        vec3<f32>(0.0, 0.0, 1.0),
    );

    var out: VertexPayload;
    out.position = vec4<f32>(positions[i], 0.0, 1.0);
    out.color = colors[i];
    return out;
}

@fragment
fn fs_main(in: VertexPayload) -> @location(0) vec4<f32> {
    // Invoke on every pixel inside the shape
    return vec4<f32>(in.color, 1.0);
}