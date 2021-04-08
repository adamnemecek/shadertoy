
// [[block]]
// struct Uniforms {
//     // width: u32;
//     // height: u32;
//     resolution: vec2<f32>;
//     time: f32;
//     // cursor_x: f32;
//     // cursor_y: f32;
//     cursor: vec2<f32>;
//     // drag_start_x: f32;
//     // drag_start_y: f32;
//     drag_start: vec2<f32>;
//     // drag_end_x: f32;
//     // drag_end_y: f32;
//     drag_end: vec2<f32>;
//     mouse_left_pressed: bool;
//     mouse_left_clicked: bool;
// };


var<push_constant> u: Uniforms;

// void mainImage( out vec4 fragColor, in vec2 fragCoord )
// {
//     // Normalized pixel coordinates (from 0 to 1)
//     vec2 uv = fragCoord/iResolution.xy;

//     // Time varying pixel color
//     vec3 col = 0.5 + 0.5*cos(iTime+uv.xyx+vec3(0,2,4));

//     // Output to screen
//     fragColor = vec4(col,1.0);
// }

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