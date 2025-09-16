/*
Purpose: WGSL shader code
Responsibilites:
    - shader.wgsl: Core vertex/fragment shader (handles transforms, colors)
    - lighting.wgsl: per-pixel lights
    - postprocess.wgsl: screen-space effects
    - texturing.wgsl: UV sampling
    - ex: The brain, runs directly on GPU, decides how stuff looks
*/

// Group 0: Camera (global)
struct Camera {
    view_proj: mat4x4<f32>,
};
@group(0) @binding(0) var<uniform> camera: Camera;

// Group 1: Per-object (model + material)
struct Uniforms {
    mvp: mat4x4<f32>,
    lit: u32,
};
struct MaterialUniform {
    use_texture: u32,
    _padding: vec3<u32>,
};
@group(1) @binding(0) var<uniform> uniforms: Uniforms;
@group(1) @binding(1) var<uniform> material: MaterialUniform;

// Group 2: Texture + Sampler (per-object optional)
@group(2) @binding(0) var myTexture: texture_2d<f32>;
@group(2) @binding(1) var mySampler: sampler;

// Group 3: Lights (global)
struct Light {
    position: vec3<f32>,
    intensity: f32,
    color: vec3<f32>,
    _padding: f32,
};
struct Lights {
    lights: array<Light, 16>,
    num_lights: u32,
};
@group(3) @binding(0) var<uniform> lights: Lights;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) color: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
    @location(3) normal: vec3<f32>,
};
struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) normal: vec3<f32>,
    @location(3) world_pos: vec3<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let world_pos = uniforms.mvp * vec4<f32>(in.position, 1.0);
    out.clip_position = camera.view_proj * world_pos;
    out.world_pos = world_pos.xyz;
    out.color = in.color;
    out.tex_coords = in.tex_coords;
    out.normal = (uniforms.mvp * vec4<f32>(in.normal, 0.0)).xyz; // transform normal properly
    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    var base_color: vec3<f32>;

    // Pick between texutre or vertex color
    if (material.use_texture == 1u) {
        let tex_color = textureSample(myTexture, mySampler, in.tex_coords);
        base_color = tex_color.rgb;
    } else {
        base_color = in.color;
    }

    // Decide if we apply lighting or not
    if (uniforms.lit == 1u) {
        var lit_color = vec3<f32>(0.0);

        // Accumulate all lights
        for (var i = 0u; i < lights.num_lights; i++) {
            let light = lights.lights[i];
            let light_dir = normalize(light.position - in.world_pos);
            let diffuse = max(dot(in.normal, light_dir), 0.0);
            lit_color += diffuse * light.color * light.intensity;
        }

        return vec4<f32>(base_color * lit_color, 1.0);
    } else {
        // unlit = just return base color
        return vec4<f32>(base_color, 1.0);
    }
    
}
