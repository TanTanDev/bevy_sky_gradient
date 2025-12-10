#import "bevy_sky_gradient/shaders/noise.wgsl"::{noise};

struct AuroraSettings {
    bottom_color: vec4<f32>, 
    color_top: vec4<f32>,
    alpha: f32, 
    density: f32, 
    sharpness: f32, 
    num_samples: i32, 
    start_height: f32, 
    end_height: f32, 
    flow_scale: f32, 
    flow_strength: f32, 
    flow_speed: f32, 
    flow_x_speed: f32, 
    wiggle_scale: f32,
    wiggle_strength: f32,
    wiggle_speed: f32,
    undersparkle_color_primary: vec4f,
    undersparkle_color_secondary: vec4f,
    undersparkle_scale: f32,
    undersparkle_speed: f32,
    undersparkle_threshold: f32,
    undersparkle_max_height: f32,
    opacity_per_sample: f32,
}

// size value expected to be between 0.0 -> 0.5
fn make_stripe(x: f32, half_size_normalized: f32) -> f32 {
    let base_value = fract(x);
    let left = smoothstep(0.5 - half_size_normalized, 0.5, base_value);
    let right = smoothstep(0.5 + half_size_normalized, 0.5, base_value);
    // let stripe = pow(left * right, settings.sharpness);
    let stripe = left * right;
    return stripe;
}

fn aurora(
    view_dir: vec3<f32>,
    settings: AuroraSettings,
    global_time: f32,
    n3_t: texture_3d<f32>,
    n3_s: sampler,
) -> vec4<f32> {
    // Ensure at least 2 samples
    let samples = max(settings.num_samples, 2);
    
    var accumulated_color = vec3<f32>(0.0);
    var accumulated_alpha = 0.0;

    let flow_time = global_time * settings.flow_speed;
    let wiggle_time = global_time * settings.wiggle_speed;
    
    // ---- Step 1: Sample along the view ray at different heights ----
    for (var i = 0; i < samples; i++) {
        let height_factor = f32(i) / f32(samples - 1);
        let height = settings.start_height + (settings.end_height - settings.start_height) * height_factor;
        
        // early exit, below horizon pixels don't need rendering
        if view_dir.y < 0.001 {
            return vec4<f32>(0.0);
        }
        let t = height / view_dir.y;
        
        let p = view_dir * t;
        let world_pos = p.xz;
        
        // global band warping
        let s_wp = (world_pos * settings.flow_scale);
        let flow = vec2<f32>(
            noise(n3_t, n3_s, vec3<f32>(s_wp.x, s_wp.y, flow_time)),
            noise(n3_t, n3_s, vec3<f32>(s_wp.x, s_wp.y, 0.5 + flow_time)));
        let flow_dir = normalize(flow);

        let w_wo = (world_pos * settings.wiggle_scale);
        // time_offset, is needed to desyncronize the timing of the wiggle. 
        // otherwise, the wiggle looks uniform accross the sky 
        let time_offset = w_wo.x + w_wo.y;
        let wiggle_noise = vec2<f32>(
            noise(n3_t, n3_s, vec3<f32>(w_wo.x, w_wo.y, wiggle_time + time_offset)),
            noise(n3_t, n3_s, vec3<f32>(w_wo.x, w_wo.y, 0.5 + wiggle_time + time_offset)));
        let wiggle = wiggle_noise * settings.wiggle_strength;
       
        let warped_pos = world_pos + flow_dir * settings.flow_strength + wiggle + settings.flow_x_speed * global_time;
        // Create 2 bands, one thicker, one less thick and spaced further away
        let large_bands = make_stripe(warped_pos.x * settings.density, 0.2);
        let smaller_bands = make_stripe(warped_pos.x * settings.density * 1.7, 0.1);
        // merge the bands
        let base_bands = pow(max(large_bands, smaller_bands), settings.sharpness);
       
        // Vertical falloff - bright at bottom, fade at top (this creates the curtain look)
        let vertical_intensity = smoothstep(0.0, 0.15, height_factor) * 
                                 smoothstep(1.0, 0.5, height_factor);

        // sparkles
        let undersparkle_intensity = 1.0 - smoothstep(0.0, settings.undersparkle_max_height, height_factor);
        let sn = (warped_pos * settings.undersparkle_scale);
        let k = global_time * settings.undersparkle_speed;
        let undersparkle_noise = smoothstep(settings.undersparkle_threshold, 1.0, 1.0-noise(n3_t, n3_s, vec3f(sn.x+k, sn.y+k, k*0.3)));
        let undersparkle_color_noise = noise(n3_t, n3_s, vec3f(sn.x*0.3+10.0, sn.y*0.3+10.0, 0.0));
        let undersparkle_color = mix(settings.undersparkle_color_primary, settings.undersparkle_color_secondary, smoothstep(0.4,1.0,undersparkle_color_noise));
        // the undersparkle is in 50% of the middle of the base band 
        let undersparkle_visibility = smoothstep(0.5, 1.0, base_bands);
        let sparkle = undersparkle_visibility * undersparkle_intensity * undersparkle_color.rgb * undersparkle_noise;
        
        var curtain = base_bands * vertical_intensity;
        
        let sample_alpha = curtain * settings.opacity_per_sample;
        let sample_weight = sample_alpha * (1.0 - accumulated_alpha);

        let selected_color = mix(settings.bottom_color.rgb, settings.color_top.rgb, height_factor);
        
        accumulated_color += selected_color * curtain * sample_weight + sparkle * vertical_intensity * sample_weight;
        accumulated_alpha += sample_alpha * (1.0 - accumulated_alpha);
        
        if (accumulated_alpha > 0.95) {
            break;
        }
    }
    
    // Fade by view angle
    let up_factor = smoothstep(0.05, 0.7, view_dir.y);
    let final_alpha = accumulated_alpha * up_factor * settings.alpha;
    
    return vec4<f32>(accumulated_color * up_factor * settings.alpha, final_alpha);
}
