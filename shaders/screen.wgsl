struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;

    let pos = array<vec4<f32>, 4>(
        vec4<f32>(-1.0, 1.0, 0.0, 1.0), // top-left
        vec4<f32>(-1.0, -1.0, 0.0, 1.0), // bottom-left
        vec4<f32>(1.0, 1.0, 0.0, 1.0), // top-right
        vec4<f32>(1.0, -1.0, 0.0, 1.0), // bottom-right
    );

    out.clip_position = pos[in_vertex_index];
    return out;
}


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.3, 0.2, 0.1, 1.0);
}
