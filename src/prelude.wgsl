

[[stage(vertex)]]
fn vs_main([[builtin(vertex_index)]] vertex_index: u32) -> [[builtin(position)]] vec4<f32> {
    // var out: vec2<f32>;

    const x = f32(i32((vertex_index << 1u32) & 2u32));
    const y = f32(i32(vertex_index & 2u32));
    const uv = vec2<f32>(x, y);
    const out = 2.0 * uv - vec2<f32>(1.0, 1.0);
    return vec4<f32>(out.x, out.y, 0.0, 1.0);

}

