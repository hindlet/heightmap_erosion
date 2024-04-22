#version 450

layout(location = 0) in vec3 f_normal;

layout(location = 0) out vec4 f_colour;

layout(set = 0, binding = 2) uniform FragUniforms {
    vec3 flat_colour;
    float slope_threshold;
    vec3 slope_colour;
    float blend_amount;
} uniforms;

void main() {
    float slope = 1 - f_normal.y;
    float blend_height = uniforms.slope_threshold * (1 - uniforms.blend_amount);
    float weight = 1 - clamp((slope - blend_height) / (uniforms.slope_threshold - blend_height), 0.0, 1.0);

    vec3 colour = weight * uniforms.flat_colour + (1-weight) * uniforms.slope_colour;
    f_colour = vec4(colour, 0);
}