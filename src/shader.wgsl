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
fn fs_main_default(
    [[builtin(position)]] frag_coord: vec4<f32>
) -> [[location(0)]] vec4<f32> {
    // return vec4<f32>(cos(u.time), sin(u.time), 1.0 - cos(u.time), 1.0);

    const uv = frag_coord.xy / u.resolution;
    const half = vec3<f32>(0.5, 0.5, 0.5);
    const time = vec3<f32>(u.time, u.time, u.time);
    const col: vec3<f32> = half + half * cos(time + uv.xyx + vec3<f32>(0.0, 2.0, 4.0));
    return vec4<f32>(col.x, col.y, col.z, 1.0);
}

// float sdRoundBox( in vec2 p, in vec2 b, in vec4 r )
// {
//     r.xy = (p.x>0.0)?r.xy : r.zw;
//     r.x  = (p.y>0.0)?r.x  : r.y;

//     vec2 q = abs(p)-b+r.x;
//     return min(max(q.x,q.y),0.0) + length(max(q,0.0)) - r.x;
// }

// void mainImage( out vec4 fragColor, in vec2 fragCoord )
// {
// 	vec2 p = (2.0*fragCoord-iResolution.xy)/iResolution.y;

// 	vec2 si = vec2(0.9,0.6);
//     vec4 ra = 0.3 + 0.3*cos( 2.0*iTime + vec4(0,1,2,3) );

// 	float d = sdRoundBox( p, si, ra );

//     vec3 col = vec3(1.0) - sign(d)*vec3(0.1,0.4,0.7);
// 	col *= 1.0 - exp(-3.0*abs(d));
// 	col *= 0.8 + 0.2*cos(150.0*d);
// 	col = mix( col, vec3(1.0), 1.0-smoothstep(0.0,0.02,abs(d)) );

// 	fragColor = vec4(col,1.0);
// }

fn sd_round_box(p: vec2<f32>, b: vec2<f32>, in_r: vec4<f32>) -> f32 {
    var r: vec4<f32>;

    r = in_r;

    if (p.x > 0.0) {
        r.x = r.x;
        r.y = r.y;
    } else {
        r.x = r.z;
        r.y = r.w;
    };
// r.x  = (p.y>0.0)?r.x  : r.y;
    if (p.y > 0.0) {
        r.x = r.x;
    } else {
        r.x = r.x;
    }
    //     r.xy = (p.x>0.0)?r.xy : r.zw;
//     r.x  = (p.y>0.0)?r.x  : r.y;
    const q = abs(p) - b + vec2<f32>(r.x, r.x);
//     vec2 q = abs(p)-b+r.x;
//     return min(max(q.x,q.y),0.0) + length(max(q,0.0)) - r.x;
     return min(max(q.x, q.y), 0.0) + length(max(q, 0.0)) - r.x;
    // return 0.0;
}

fn splat2(v: f32) -> vec2<f32> {
    return vec2<f32>(v, v);
}

fn splat3(v: f32) -> vec3<f32> {
    return vec3<f32>(v, v, v);
}

fn splat4(v: f32) -> vec4<f32> {
    return vec4<f32>(v, v, v, v);
}

fn mix3(a: vec3<f32>, b: vec3<f32>, d: f32) -> vec3 {
    return vec3<f32>(
        mix(a.x, b.x, d),
        mix(a.y, b.y, d)
        mix(a.z, b.z, d)
    );
}

[[stage(fragment)]]
fn fs_main(
    [[builtin(position)]] frag_coord: vec4<f32>
) -> [[location(0)]] vec4<f32> {
    // return vec4<f32>(cos(u.time), sin(u.time), 1.0 - cos(u.time), 1.0);

    // const uv = frag_coord.xy / u.resolution;
    // const half = vec3<f32>(0.5, 0.5, 0.5);
    // const time = vec3<f32>(u.time, u.time, u.time);
    // const col: vec3<f32> = half + half * cos(time + uv.xyx + vec3<f32>(0.0, 2.0, 4.0));
    // return vec4<f32>(col.x, col.y, col.z, 1.0);

    // const p =
    // const two = vec2<f32>(2.0, 2.0);
    const p = (splat2(2.0)*frag_coord.xy-u.resolution)/splat2(u.resolution.y);

    const si = vec2<f32>(0.9, 0.6);
	// vec2 si = vec2(0.9,0.6);
    // vec4 ra = 0.3 + 0.3*cos( 2.0*iTime + vec4(0,1,2,3) );
    // const third = vec4<f32>(0.3, 0.3, 0.3, 0.3);
    // const t = vec2<f32>()
    const ra = splat4(0.3) + splat4(0.3) * cos(splat4(u.time) * vec4<f32>(0.0, 1.0, 2.0, 3.0));
    const d = sd_round_box(p, si, ra);
	// float d = sdRoundBox( p, si, ra );
    const col = splat3(1.0) - sign(d)  * vec3<f32>(0.1, 0.4, 0.7);
    const col1 = col * splat3(1.0) - splat3(exp(-3.0 * abs(d)));
    const col2 = col1 * splat3(0.8) * splat3(0.2) * cos(150.0 * d);
    const col3 = mix3(col2, splat3(1.0), 1.0 - smoothStep(0.0, 0.02, abs(d)));

    // vec3 col = vec3(1.0) - sign(d)*vec3(0.1,0.4,0.7);
	// col *= 1.0 - exp(-3.0*abs(d));
	// col *= 0.8 + 0.2*cos(150.0*d);
	// col = mix( col, vec3(1.0), 1.0-smoothstep(0.0,0.02,abs(d)) );

	// fragColor = vec4(col,1.0);
    // return vec4<f32>(0.5, 0.5, 0.5, 0.5);
    return vec4<f32>(col3, 1.0);

}