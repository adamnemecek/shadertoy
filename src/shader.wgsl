[[block]]
struct Uniforms {
    resolution: vec2<f32>;
    frame: f32;
    time: f32;
    cursor: vec2<f32>;
    drag_start: vec2<f32>;
    drag_end: vec2<f32>;
    mouse_left_pressed: bool;
    mouse_left_clicked: bool;
};

var<push_constant> u: Uniforms;

[[stage(fragment)]]
fn fs_main(
    [[builtin(position)]] frag_coord: vec4<f32>
) -> [[location(0)]] vec4<f32> {
    // return vec4<f32>(cos(u.time), sin(u.time), 1.0 - cos(u.time), 1.0);

    const uv = frag_coord.xy / u.resolution;
    const half = vec3<f32>(0.5, 0.5, 0.5);
    const time = vec3<f32>(u.time, u.time, u.time);
    const col: vec3<f32> = half + half * cos(time + uv.xyx + vec3<f32>(0.0, 2.0, 4.0));
    return vec4<f32>(col.x, col.y, col.z, 1.0);
}