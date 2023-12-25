#version 460

struct Vertex {
    dvec3 position;
    dvec3 velocity;
};

layout(set = 0, binding = 0) uniform DataBlock {
    double min;
    double max;
    uvec4 length;
}inputData;

layout(set = 0, binding = 1) readonly buffer InputBlock {
    Vertex array[];
}inputBuffer;

layout(set = 1, binding = 0) buffer OutputBlock {
    uint array[];
}outputBuffer;

void main() {
    uvec3 id = gl_GlobalInvocationID;
    uint index = id.x * inputData.length.x + id.y;

    if (id.y <= id.x) {
        outputBuffer.array[index] = 0;
        return;
    }

    Vertex vertices[2];
    vertices[0] = inputBuffer.array[id.x];
    vertices[1] = inputBuffer.array[id.y];

    dmat2 matrix;
    dvec2 constv;

    matrix[1] = dvec2(1.0, 1.0);

    for (int i = 0; i < 2; i++) {
        matrix[0][i] = -1 * vertices[i].velocity.y / vertices[i].velocity.x;
        constv[i] = vertices[i].position.y - (vertices[i].position.x * vertices[i].velocity.y) / vertices[i].velocity.x;
    }

    dvec2 res = inverse(matrix) * constv;

    outputBuffer.array[index] = uint(
        !isnan(res.x) 
        && ((res.x - vertices[0].position.x) / vertices[0].velocity.x) > 0
        && ((res.x - vertices[1].position.x) / vertices[1].velocity.x) > 0
        && (res.x >= inputData.min && res.x <= inputData.max && res.y >= inputData.min && res.y <= inputData.max) 
    );
}
