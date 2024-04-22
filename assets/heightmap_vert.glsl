#version 460

layout(set = 0, binding = 2) buffer HeightMap {
    float[] heightmap;
};

layout(location = 0) in ivec2 heightmap_pos;

layout(location = 0) out vec3 f_normal;

layout(set = 0, binding = 0) uniform VertUniforms {
    float step_size;
    int width;
    int height;
    mat4 view;
    mat4 proj;
} uniforms;


void main() {
    // normal calculations
    int index = heightmap_pos.y * uniforms.width + heightmap_pos.x;
    vec3 self_pos = vec3(heightmap_pos.x * uniforms.step_size, heightmap[index], heightmap_pos.y * uniforms.step_size);
    vec3 point_one = vec3((heightmap_pos.x - 1) * uniforms.step_size, heightmap[index - 1], heightmap_pos.y * uniforms.step_size);
    vec3 point_two = vec3(heightmap_pos.x * uniforms.step_size, heightmap[index + uniforms.width], (heightmap_pos.y + 1) * uniforms.step_size);
    vec3 point_three = vec3((heightmap_pos.x + 1) * uniforms.step_size, heightmap[index + 1], heightmap_pos.y * uniforms.step_size);
    vec3 point_four = vec3(heightmap_pos.x * uniforms.step_size, heightmap[index - uniforms.width], (heightmap_pos.y - 1) * uniforms.step_size);
    vec3 normal = cross(point_one, point_two) + cross(point_two, point_three) + cross(point_three, point_four) + cross(point_four, point_one);

    // actual vert stuff
    gl_Position = uniforms.proj * uniforms.view * vec4(self_pos, 1.0);
    f_normal = normalize(normal);
}