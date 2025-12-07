struct SunSettings {
    sun_dir: vec3<f32>,
    sun_color: vec4<f32>,
    sun_strength: f32,
    sun_sharpness: f32,
}

fn sun(view_dir: vec3f, sun: SunSettings) -> vec4f {
    let sun_factor = pow(max(dot(view_dir, normalize(sun.sun_dir)), 0.0), sun.sun_sharpness);
    return sun.sun_color * (sun_factor * sun.sun_strength);
}
