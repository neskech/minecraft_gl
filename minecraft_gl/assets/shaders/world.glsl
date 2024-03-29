#type vertex
#version 410 core

layout (location=0) in uint Core;
layout (location=1) in uint Dims;

uniform mat4 proj;
uniform mat4 view;

uniform vec2 chunk_pos;
uniform float atlas_cols;

out vec2 fuv_top;
out vec2 fuv_width;
out float faceID;
out float tile_size;

const vec2 offsets[4] = vec2[4](
    vec2(0, 0), vec2(1, 0),
    vec2(0, 1), vec2(1, 1)
);

void main(){
    // X (4), Y(4) ,   Z(8)   , TexId(8) , QuadId(2),  FaceId(3)
    // 0000 | 0000 | 00000000 | 00000000 | 00 | 000

    float x = float(Core & 0xFu) + chunk_pos.x * 15.0;
    float z = float( (Core >> 4u) & 0xFu ) + chunk_pos.y * 15.0;
    float y = float( (Core >> 8u) & 0xFFu );

    vec2 fdims = vec2(float(Dims & 0xFFFFu), float((Dims >> 16u) & 0xFFFFu));

    uint texID = (Core >> 16u) & 0xFFu; //8 bits
    uint quadID = (Core >> 24u) & 0x3u; //2 bits
    faceID = float((Core >> 26u) & 0x7u); //3 bits

    float row = floor(float(texID) / atlas_cols);
    float col = float(texID % uint(atlas_cols));
    float tile_dims = 1.0 / atlas_cols;
    tile_size = tile_dims;

    vec2 top_left_uv = vec2(col * tile_dims, row * tile_dims);
    //top_left_uv += offsets[quadID] * tile_dims;
    top_left_uv.y = 1.0 - top_left_uv.y;
    fuv_top = top_left_uv;
    fuv_width = offsets[quadID] * fdims * tile_dims;

    gl_Position = proj * view * vec4(x, y, z, 1.0);
}

#type fragment
#version 410 core

uniform sampler2D atlas;


in vec2 fuv_top;
in vec2 fuv_width;
in float tile_size;
in float faceID;

const float values[6] = float[6](
   0.2, 0.2, 1.0, 0.3, 0.2, 0.2
);


const vec3 normals[6] = vec3[6](
    vec3(1, 0, 0), vec3(-1, 0, 0), //Right, Left
    vec3(0, 1, 0), vec3(0, -1, 0), //Up, Down
    vec3(0, 0, 1), vec3(0, 0, -1) //Forward, Backward

);

out vec4 Color;

void main(){
      vec4 val = texture(atlas, vec2(fuv_top.x + mod(fuv_width.x, tile_size), fuv_top.y - mod(fuv_width.y, tile_size)));
      //vec4 val = texture(atlas, vec2(fuv_top.x, fuv_top.y));
      float mult = values[int(faceID)];
      Color = vec4(val.x * mult, val.y * mult, val.z * mult, val.w);
      //Color = vec4(0.6 * mult, 0.1 * mult, 0.3 * mult, 1.0);
}
