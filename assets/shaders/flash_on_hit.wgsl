#import bevy_sprite::mesh2d_vertex_output::VertexOutput
// we can import items from shader modules in the assets folder with a quoted path

@group(2) @binding(0) var<uniform> on_hit_color: vec4<f32>;
@group(2) @binding(1) var base_color_texture: texture_2d<f32>;
@group(2) @binding(2) var base_color_sampler: sampler;
@group(2) @binding(3) var<uniform> is_attacked: u32;


@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let sampled_texture = textureSample(base_color_texture, base_color_sampler, mesh.uv);

    if (is_attacked == 1u) {
        return on_hit_color * sampled_texture;
    }

    return sampled_texture;
}