#type vertex
#version 410 core

layout (location=0) in uint Data;

uniform mat4 proj;
uniform mat4 view;

uniform vec2 chunk_pos;
uniform float atlas_cols;
uniform vec2 sprite_dimensions;
uniform vec2 texSize;

out vec2 fuvs;
out float faceID;

const vec2 offsets[4] = vec2[4](
    vec2(1, 1), vec2(0, 1),
    vec2(1, 0), vec2(0, 0)
);

void main(){
    // X (4), Y(4) ,   Z(8)   , TexId(8) , QuadId(2),  FaceId(3)
    // 0000 | 0000 | 00000000 | 00000000 | 00 | 000

    float x = float(Data & 0xFu) + chunk_pos.x * 8.0;
    float z = float( (Data >> 4u) & 0xFu ) + chunk_pos.y * 8.0;
    float y = float( (Data >> 8u) & 0xFFu );

    uint texID = (Data >> 16u) & 0xFFu; //8 bits
    uint quadID = (Data >> 24u) & 0x3u; //2 bits
    faceID = float((Data >> 26u) & 0x7u); //3 bits

    float row = floor(float(texID) / atlas_cols);
    float col = float(texID % uint(atlas_cols));

    vec2 top_left_uv = vec2(col * sprite_dimensions.x, row * sprite_dimensions.y);
    top_left_uv = (top_left_uv + offsets[quadID] * sprite_dimensions) / texSize;
    top_left_uv.y = 1.0 - top_left_uv.y;
    fuvs = top_left_uv;

    gl_Position = proj * view * vec4(x, y, z, 1.0);
}

#type fragment
#version 410 core

uniform sampler2D atlas;

// in int texID;
// in int quadID;
// in int faceID;
in vec2 fuvs;
in float faceID;

out vec4 Color;

void main(){


      Color = texture(atlas, fuvs);
      //Color = vec4(1.0, 0.0, 1.0, 1.0);
}
