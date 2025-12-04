struct GradientSettings {
    color_stops: array<vec4<f32>, 4>,
    positions: vec4<f32>,
    num_stops: u32,
}

fn gradient(view_dir: vec3<f32>, gradient_settings: GradientSettings) -> vec4<f32> {
    let t = clamp(view_dir.y * 0.5 + 0.5, 0.0, 1.0);
    // GRADIENT
    var base_color: vec4<f32> = gradient_settings.color_stops[0];
    // if below first stop
    if (t <= gradient_settings.positions[0]) {
        base_color = gradient_settings.color_stops[0];
    }
    // if above last stop
    else if (t >= gradient_settings.positions[gradient_settings.num_stops - 1u]) {
        base_color = gradient_settings.color_stops[gradient_settings.num_stops - 1u];
    }
    // otherwise, find segment and interpolate
    else {
        for (var i: u32 = 1u; i < gradient_settings.num_stops; i = i + 1u) {
            let a = gradient_settings.positions[i - 1u];
            let b = gradient_settings.positions[i];
            if (t >= a && t <= b) {
                let f = (t - a) / (b - a);
                base_color = mix(gradient_settings.color_stops[i - 1u], gradient_settings.color_stops[i], f);
                break;
            }
        }
    }
    return base_color;
}
