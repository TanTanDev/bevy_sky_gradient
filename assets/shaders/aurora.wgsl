#import "shaders/noise.wgsl"::{noise};

struct AuroraSettings {
    bottom_color: vec4<f32>, 
    alpha: f32, 
    density: f32, 
    sharpness: f32, 
    num_samples: i32, 
    start_height: f32, 
    end_height: f32, 
    flow_scale: f32, 
    flow_strength: f32, 
    flow_speed: f32, 
    wiggle_scale: f32,
    wiggle_strength: f32,
    wiggle_speed: f32,
    color_top: vec4<f32>,
    undersparkle_color_primary: vec4f,
    undersparkle_color_secondary: vec4f,
    undersparkle_scale: f32,
    undersparkle_speed: f32,
    undersparkle_threshold: f32,
    undersparkle_max_height: f32,
}

fn aurora(view_dir: vec3<f32>, settings: AuroraSettings, global_time: f32, n3_t: texture_3d<f32>, n3_s: sampler) -> vec4<f32> {
    // Ensure at least 2 samples
    let samples = max(settings.num_samples, 2);
    
    var accumulated_color = vec3<f32>(0.0);
    var accumulated_alpha = 0.0;
    
    // ---- Step 1: Sample along the view ray at different heights ----
    for (var i = 0; i < samples; i++) {
        let height_factor = f32(i) / f32(samples - 1);
        let height = settings.start_height + (settings.end_height - settings.start_height) * height_factor;
        
        // Find where THIS ray hits THIS specific height plane
        let denom = view_dir.y;
        if abs(denom) < 0.001 {
            return vec4<f32>(0.0);
        }
        let t = height / denom;
        if (t <= 0.0) {
            continue;
        }
        
        let p = view_dir * t;
        let world_pos = p.xz;
        
        let s_wp = world_pos * settings.flow_scale;
        let flow = vec2<f32>(
            noise(n3_t, n3_s, vec3<f32>(s_wp.x, s_wp.y, 1.0+ global_time * settings.flow_speed)),
            noise(n3_t, n3_s, vec3<f32>(s_wp.x, s_wp.y, 7.7+ global_time * settings.flow_speed)));

        let w_wo = world_pos * settings.wiggle_scale;
        let time_offset = w_wo.x+w_wo.y;
        var wiggle = vec2<f32>(
            noise(n3_t, n3_s, vec3<f32>(w_wo.x, w_wo.y, 10.0 + global_time * settings.wiggle_speed+time_offset)),
            noise(n3_t, n3_s, vec3<f32>(w_wo.x, w_wo.y, 70.7 + global_time * settings.wiggle_speed+time_offset)));
        
        wiggle = wiggle * settings.wiggle_strength;
        // let flow_dir = normalize(flow * 2.0 - 1.0);
        let flow_dir = normalize(flow);
       
        let warped_pos = world_pos + flow_dir * settings.flow_strength + wiggle;
        // Create bands
        let band_coord = warped_pos.x * settings.density;
        // let band_coord = (rotated_pos.x * density + dance_offset + band_noise) * wave_count;
        let base_value = fract(band_coord);

        let stripe = pow(smoothstep(0.3, 0.5, base_value) * smoothstep(0.7, 0.5, base_value), settings.sharpness);
        
        // Secondary bands 50% weaker, but 3 times thinner, and more frequent
        let secondary_value = fract(band_coord * 1.7 + 0.3);
        let secondary = pow(smoothstep(0.35, 0.5, secondary_value) * smoothstep(0.65, 0.5, secondary_value), settings.sharpness * 3.0);
        
        let base_stripe = max(stripe, secondary * 0.5);
       
        // Vertical falloff - bright at bottom, fade at top (this creates the curtain look)
        let vertical_intensity = smoothstep(0.0, 0.15, height_factor) * 
                                 smoothstep(1.0, 0.5, height_factor);

        // sparkles
        let undersparkle_intensity = 1.0 - smoothstep(0.0, settings.undersparkle_max_height, height_factor);
        let sn = warped_pos * settings.undersparkle_scale;
        let k = global_time * settings.undersparkle_speed;
        let undersparkle_noise = smoothstep(settings.undersparkle_threshold, 1.0, 1.0-noise(n3_t, n3_s, vec3f(sn.x+k, sn.y+k, k*0.3)));
        let undersparkle_color_noise = noise(n3_t, n3_s, vec3f(sn.x*0.3+10.0, sn.y*0.3+10.0, 0.0));
        let undersparkle_color = mix(settings.undersparkle_color_primary, settings.undersparkle_color_secondary, smoothstep(0.4,1.0,undersparkle_color_noise));
        let sparkle = pow(base_stripe,3.0) * undersparkle_intensity * undersparkle_color.rgb * undersparkle_noise;
        
        var curtain = base_stripe * vertical_intensity;
        
        // ---- Step 4: Accumulate with proper alpha blending ----
        let sample_alpha = curtain * 0.18;
        let sample_weight = sample_alpha * (1.0 - accumulated_alpha);

        let selected_color = mix(settings.bottom_color.rgb, settings.color_top.rgb, height_factor);
        
        accumulated_color += selected_color * curtain * sample_weight + sparkle;
        // accumulated_color += bottom_color.rgb * curtain * sample_weight;
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
