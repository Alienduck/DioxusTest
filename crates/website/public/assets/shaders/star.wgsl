#import bevy_pbr::mesh_view_bindings::globals
#import bevy_pbr::forward_io::VertexOutput

struct StarSettings {
    color: vec4<f32>,
    intensity: f32,
    phase: f32,
    speed: f32,
}

@group(1) @binding(0) var<uniform> settings: StarSettings;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let twinkle = (sin(globals.time * settings.speed + settings.phase) * 0.4) + 0.6;
    let final_color = settings.color.rgb * (settings.intensity * twinkle);
    return vec4<f32>(final_color, 1.0);
}
