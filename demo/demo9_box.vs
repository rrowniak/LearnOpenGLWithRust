#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTextCoords;

out vec3 FragPos;
out vec3 Normal;
out vec2 TexCoords;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    FragPos = vec3(model * vec4(aPos, 1.0));
	// slow but general
	Normal = mat3(transpose(inverse(model))) * aNormal;
	// fast but limited
    // Normal = aNormal;

    TexCoords = aTextCoords; 
    
    gl_Position = projection * view * vec4(FragPos, 1.0);
}