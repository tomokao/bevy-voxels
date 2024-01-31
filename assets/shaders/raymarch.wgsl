#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(1) @binding(0) var<uniform> pos: vec3f;
@group(1) @binding(1) var<uniform> rotation: vec4f;
@group(1) @binding(2) var<uniform> inverse_camera: mat4x4f;
@group(1) @binding(3) var<uniform> resolution: vec2f;

fn get_gradient(intPos: vec2f, t: f32) -> vec2f {
    let rand = fract(sin(dot(intPos, vec2f(12.9898, 78.233))) * 43758.5453);
    let angle = 6.283185 * rand + 4.0 * t * rand;
    return vec2f(cos(angle), sin(angle));
}

fn pseudo_3d_noise(pos: vec3f) -> f32 {
    let i = floor(pos.xy);
    let f = pos.xy - i;
    let blend = f * f * (3.0 - 2.0 * f);
    var noiseVal = mix(
        mix(
            dot(get_gradient(i + vec2f(0.0, 0.0), pos.z), f - vec2f(0.0, 0.0)),
            dot(get_gradient(i + vec2f(1.0, 0.0), pos.z), f - vec2f(1.0, 0.0)),
            blend.x
        ),
        mix(
            dot(get_gradient(i + vec2f(0.0, 1.0), pos.z), f - vec2f(0.0, 1.0)),
            dot(get_gradient(i + vec2f(1.0, 1.0), pos.z), f - vec2f(1.0, 1.0)),
            blend.x
        ),
        blend.y
    );
    return noiseVal / 0.7; // [-1..1]
}

fn is_hit(pos: vec3i) -> bool {
    return pseudo_3d_noise(vec3f(pos) / 30.0) > 0.0;
    //return distance(vec3f(pos), vec3f()) < 5.0;
}

fn apply_rotation(vec: vec3f) -> vec3f {
    let t = 2.0 * cross(rotation.xyz, vec);
    return vec + rotation.w * t + cross(rotation.xyz, t);
}

fn march(start_pos: vec3f, direction: vec3f, max_distance: f32) -> vec4f {
    var marched = 0.0;
    while marched <= max_distance {
        let pos = start_pos + direction * marched;
        let pos_i = vec3i(pos);
        if is_hit(pos_i) {
            let color = vec3f(1.0, 1.0, 1.0);
            //return vec4f((max_distance - marched) / max_distance * color, 1.0);
            return vec4f(pow(max_distance - distance(pos, start_pos), 6.0) / pow(max_distance, 6.0) * color, 1.0);
        }

        let deltas = (step(vec3f(), direction) - fract(pos)) / direction;
        marched += max(min(min(deltas.x, deltas.y), deltas.z), 0.0005);
    }
    return vec4f(0.0, 0.0, 0.0, 1.0);
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4f {
    var uv = mesh.uv * 2.0 - 1.0;
    uv.x = -uv.x;
    let direction = normalize(vec3f(uv, 1.0));

    return march(-pos, apply_rotation(direction), 200.0);
}
