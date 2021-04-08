
struct Uniforms {
    width: u32,
    height: u32,
    time: f32,
    cursor_x: f32,
    cursor_y: f32,
    drag_start_x: f32,
    drag_start_y: f32,
    drag_end_x: f32,
    drag_end_y: f32,
    mouse_left_pressed: bool,
    mouse_left_clicked: bool,
};

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return textureSample(r_color, r_sampler, in.tex_coord);
}