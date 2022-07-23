// Vertex shader

struct Uniform {
    view_proj: mat4x4<f32>,
    chunk_pos: vec2<f32>,
    sprite_dimensions: f32,
    tex_size: vec2<f32>,
    atlas_cols: f32,
};

@group(1) @binding(0) // 1.
var<uniform> udata: Uniform;

struct VertexInput {
    @location(0) data: uint,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;

    var x: f32 = (model.data & 0xFu) as f32 + udata.chunk_pos.x * 15.0;
    var z: f32 = float( (model.data >> 4u) & 0xFu ) + udata.chunk_pos.y * 15.0;
    var y: f32 = float( (model.data >> 8u) & 0xFFu );

    var texID: u32 = (model.data >> 16u) & 0xFFu; //8 bits
    var quadID: u32 = (model.data >> 24u) & 0x3u; //2 bits
    var faceID: f32 = float((model.data >> 26u) & 0x7u); //3 bits

    var row: f32 = floor(float(texID) / atlas_cols);
    var col: f32 = float(texID % uint(atlas_cols));

    var top_left_uv: vec2<f32> = vec2(col * udata.sprite_dimensions.x, row * udata.sprite_dimensions.y);
    top_left_uv = (top_left_uv + offsets[quadID] * udata.sprite_dimensions) / udata.tex_size;
    top_left_uv.y = 1.0 - top_left_uv.y;

    out.uv = top_left_uv;
    out.clip_position = udata.view_proj * vec4(x, y, z);

    return out;
}

// Fragment shader
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0)@binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}