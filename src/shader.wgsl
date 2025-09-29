/*
Purpose: WGSL shader code
Responsibilites:
    - shader.wgsl: Core vertex/fragment shader (handles transforms, colors)
    - lighting.wgsl: per-pixel lights
    - postprocess.wgsl: screen-space effects
    - texturing.wgsl: UV sampling
    - ex: The brain, runs directly on GPU, decides how stuff looks
*/

// // Group 0: Camera (global)
// struct Camera {
//     view_proj: mat4x4<f32>,
// };
// @group(0) @binding(0) var<uniform> camera: Camera;

// struct VertexInput {
//     @location(0) position: vec3<f32>,
//     @location(1) color: vec3<f32>,
//     @location(2) tex_coords: vec2<f32>,
//     @location(3) normal: vec3<f32>,
// };

// Group 0: Texture/Sampler
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

// Group 1: Camera
struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

struct InstanceInput {
    @location(5) model_matrix_0: vec4<f32>,
    @location(6) model_matrix_1: vec4<f32>,
    @location(7) model_matrix_2: vec4<f32>,
    @location(8) model_matrix_3: vec4<f32>,
};


// Grabbing data from the vertex buffer
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
}

// store the output of the vertex shader
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    let model_matrix = mat4x4<f32>(
        instance.model_matrix_0,
        instance.model_matrix_1,
        instance.model_matrix_2,
        instance.model_matrix_3,
    );

    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = camera.view_proj * model_matrix * vec4<f32>(model.position, 1.0);
    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_coords);
}
