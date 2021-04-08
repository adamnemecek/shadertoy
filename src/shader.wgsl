
var<push_constant> u: Uniforms;


[[stage(fragment)]]
fn fs_main() -> [[location(0)]] vec4<f32> {
    return vec4<f32>(0.0, sin(u.time), 0.0, 1.0);
}