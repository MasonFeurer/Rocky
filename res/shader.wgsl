// Vertex shader

struct VertexInput {
	@location(0) pos: vec3<f32>,
	@location(1) tex_coords: vec2<f32>,
}
struct VertexOutput {
	@builtin(position) clip_position: vec4<f32>,
	@location(0) tex_coords: vec2<f32>,
}

@group(1) @binding(0)
var<uniform> view_mat: mat4x4<f32>;

@group(1) @binding(1)
var<uniform> proj_mat: mat4x4<f32>;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
	var out: VertexOutput;
	
	out.clip_position = proj_mat * view_mat * vec4<f32>(in.pos, 1.0);
	out.tex_coords = in.tex_coords;
	
	return out;
}

// Fragment shader

@group(0) @binding(0)
var texture: texture_2d<f32>;
@group(0) @binding(1)
var texture_s: sampler;

@fragment
fn fs_main(
	in: VertexOutput
) -> @location(0) vec4<f32> {
	return textureSample(texture, texture_s, in.tex_coords);
}
