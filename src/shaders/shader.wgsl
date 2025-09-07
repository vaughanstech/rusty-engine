/*
Purpose: WGSL shader code
Responsibilites:
    - shader.wgsl: Core vertex/fragment shader (handles transforms, colors)
    - lighting.wgsl: per-pixel lights
    - postprocess.wgsl: screen-space effects
    - texturing.wgsl: UV sampling
    - ex: The brain, runs directly on GPU, decides how stuff looks
*/

// Vertex shader: outputs position + color
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec3<f32>,
};
struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec3<f32>,
};

// Vertex shader: takes position and passes it along
// Update: adding uniform matrix that multiplies each vertex
struct Uniforms {
    transform: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(
    in: VertexInput
) -> VertexOutput {
    var out: VertexOutput;
    out.position = uniforms.transform * vec4<f32>(in.position, 0.0, 1.0);
    out.color = in.color;
    return out;
}

// Fragment shader: takes color from vertex shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}