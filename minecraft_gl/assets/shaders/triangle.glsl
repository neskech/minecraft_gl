#type vertex
#version 410 core

layout (location=0) in vec3 pos;

uniform mat4 proj;
uniform mat4 view;



void main(){

    gl_Position = proj * view *  vec4(pos.x, pos.y, pos.z, 1.0);
}

#type fragment
#version 410 core


out vec4 Color;

void main(){

      Color = vec4(1.0, 0.5, 0.4, 1.0);
}
