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

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
};
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

// Vertex shader: takes position and passes it along
// Update: adding uniform matrix that multiplies each vertex
struct Uniforms {
    mvp: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@group(2) @binding(0)
var myTexture: texture_2d<f32>;

@group(2) @binding(1)
var mySampler: sampler;


struct Camera {
    view_proj: mat4x4<f32>,
};

// Shader needs to accept the camera matrix
@group(1) @binding(0)
var<uniform> camera: Camera;

@vertex
fn vs_main(
    in: VertexInput
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = uniforms.mvp * vec4<f32>(in.position, 1.0);
    out.color = in.color;
    out.tex_coords = in.tex_coords;
    return out;
}

// Fragment shader: takes color from vertex shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let tex_color = textureSample(myTexture, mySampler, in.tex_coords);
    return tex_color * vec4(in.color, 1.0);
}