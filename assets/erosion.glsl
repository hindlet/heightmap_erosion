#version 460
#define CHECKLEFT index % push_constants.width == 0
#define CHECKRIGHT (index + 1) % push_constants.width == 0
#define CHECKDOWN index / push_constants.width == 0
#define CHECKUP (index + push_constants.width) / push_constants.width == push_constants.height


layout(local_size_x = 32, local_size_y = 32, local_size_z = 1) in;

// MIDDLE OF A SQUARE - VERTICES AT CORNERS
// left: -x
// right: +x
// down: -y
// up: +y
struct CellInfo {
    vec4 outflow_flux; // left, right, down, up
    vec4 velocity;
    float terrain_height;
    float water_height;
    float suspended_sediment;
    float total_height;
};

layout(set = 0, binding = 0) buffer HeightmapInBuffer {
    CellInfo[] heightmap;
};





layout(push_constant) uniform PushConstants {
    // 0 - Add water
    // 1 - Init out buffer
    // 2 - Flow simulate
    // 3 - Deposit and Erode
    uint step;

    int width;
    int height;
    vec2 cell_size;

    float timestep;
    float pipe_area;
    float pipe_length;
    float gravity;
} push_constants;


int get_index(uint x, uint y) {
    return int(x) + int(y) * push_constants.width;
}

ivec2 break_index(int index) {
    return ivec2(index % push_constants.width, index / push_constants.width);
}


// void add_water() {
//     // get index
//     if gl_GlobalInvocationID.x > push_constants.width || gl_GlobalInvocationID.y > push_constants.height {
//         return;
//     }
//     int index = gl_GlobalInvocationID.x + gl_GlobalInvocationID.y * push_constants.width;

//     // add water
//     // OutHeightmap[index].water_height = x
// }

// void init() {
//     // get index
//     if gl_GlobalInvocationID.x > push_constants.width || gl_GlobalInvocationID.y > push_constants.height {
//         return;
//     }
//     int index = get_index(gl_GlobalInvocationID.x, gl_GlobalInvocationID.y);
    
//     // copy in the flow and heights
//     OutHeightmap[index].outflow_flux = InHeightmap[index].outflow_flux
// }


vec4 get_height_differences(int index) {

    float self_height = heightmap[index].total_height;
    float left_height = (CHECKLEFT) ? self_height : heightmap[index - 1].total_height;
    float right_height = (CHECKRIGHT) ? self_height : heightmap[index + 1].total_height;
    float down_height = (CHECKDOWN) ? self_height : heightmap[index - push_constants.width].total_height;
    float up_height = (CHECKUP) ? self_height : heightmap[index + push_constants.width].total_height;


    return vec4(
        self_height - left_height,
        self_height - right_height,
        self_height - down_height,
        self_height - up_height
    );
}

void flow_simulate() {
    // get index
    if (gl_GlobalInvocationID.x > push_constants.width || gl_GlobalInvocationID.y > push_constants.height) {
        return;
    }
    int index = get_index(gl_GlobalInvocationID.x, gl_GlobalInvocationID.y);


    // calculate new outflow_flux
    vec4 height_diffs = get_height_differences(index);
    vec4 current_flow = heightmap[index].outflow_flux;
    float flow_multiplier = push_constants.timestep * push_constants.pipe_area * push_constants.gravity / push_constants.pipe_length;

    float flow_left = max(0, current_flow.x + height_diffs.x * flow_multiplier);
    float flow_right = max(0, current_flow.y + height_diffs.y * flow_multiplier);
    float flow_down = max(0, current_flow.z + height_diffs.z * flow_multiplier);
    float flow_up = max(0, current_flow.w + height_diffs.w * flow_multiplier);
    float flow_sum = flow_left + flow_right + flow_up + flow_down;

    float flow_scale = min(1, heightmap[index].water_height * push_constants.cell_size.x * push_constants.cell_size.y / (flow_sum * push_constants.timestep));

    vec4 new_flow = vec4(flow_left, flow_right, flow_down, flow_up) * flow_scale;

    heightmap[index].outflow_flux = new_flow;

    // float flow_left = max(0.0, out_data.outflow_flux.x * push_constants.timestep * push_constants.pipe_area * push_constants.gravity)
}


void adjust_water_height_and_flow_velocity() {
    // get index
    if (gl_GlobalInvocationID.x > push_constants.width || gl_GlobalInvocationID.y > push_constants.height) {
        return;
    }
    int index = get_index(gl_GlobalInvocationID.x, gl_GlobalInvocationID.y);

    float cell_area = push_constants.cell_size.x * push_constants.cell_size.y;
    vec4 self_flow = heightmap[index].outflow_flux;
    vec4 outflow = push_constants.timestep * self_flow / cell_area;

    // left flow
    if (CHECKLEFT) {
        heightmap[index - 1].water_height += outflow.x;
    }

    // right flow
    if (CHECKRIGHT) {
        heightmap[index + 1].water_height += outflow.y;
    }

    // down flow
    if (CHECKDOWN) {
        heightmap[index - push_constants.width].water_height += outflow.z;
    }

    // up flow
    if (CHECKUP) {
        heightmap[index + push_constants.width].water_height += outflow.w;
    }

    heightmap[index].water_height -= outflow.x + outflow.y + outflow.z + outflow.w;

    // velocity
    float double_delta_x = ((CHECKLEFT)? 0 : heightmap[index - 1].outflow_flux.y) - self_flow.x + self_flow.y - ((CHECKRIGHT)? 0 : heightmap[index + 1].outflow_flux.x);
    float double_delta_y = ((CHECKDOWN)? 0 : heightmap[index - push_constants.width].outflow_flux.w) - self_flow.z + self_flow.w - ((CHECKUP)? 0 : heightmap[index + push_constants.width].outflow_flux.z);

    heightmap[index].velocity = vec4(double_delta_x, double_delta_y, 0, 0) / 2.0;
}


void erode_and_deposit() {
    // get index
    if (gl_GlobalInvocationID.x > push_constants.width || gl_GlobalInvocationID.y > push_constants.height) {
        return;
    }
    int index = get_index(gl_GlobalInvocationID.x, gl_GlobalInvocationID.y);




}


void main() {
    if (push_constants.step == 2) {
        flow_simulate();
    }
}
