
var<push_constant> u: Uniforms;

[[stage(fragment)]]
fn fs_main(
    [[builtin(position)]] frag_coord: vec4<f32>
) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(cos(u.time), sin(u.time), 1.0 - cos(u.time), 1.0);
}