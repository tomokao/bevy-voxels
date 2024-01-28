#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(mesh.uv.x, 0.0, mesh.uv.y, 1.0);
}
