#import "shaders/noise.wgsl"::{noise};

struct StarsSettings {
    sky_rotation_speed: f32,
    sample_scale: f32,
    star_threshold: f32,
    star_threshold_blink: f32,
    blink_speed: f32,
    mask_scale: f32,
    mask_threshold: f32,
    blink_variance_scale: f32,
}

fn stars(
    view_dir: vec3<f32>,
    stars: StarsSettings,
    global_time: f32,
   // noise and voronoi noise
    n3_t: texture_3d<f32>,
    n3_s: sampler,
    v3_t: texture_3d<f32>,
    v3_s: sampler,
) -> f32 {
    let sky_rotation = global_time * stars.sky_rotation_speed;
    let c = cos(sky_rotation);
    let s = sin(sky_rotation);
    let rotation_matrix = mat3x3<f32>(
        vec3<f32>(c,-s,0.0), // new x basis vector
        vec3<f32>(s, c, 0.0), // new y basis vector
        vec3<f32>(0.0,0.0,1.0), // new z basis vector
    );
    let offset_world_dir = rotation_matrix * view_dir;

    var noise = 1.0-noise(v3_t, v3_s, offset_world_dir * stars.sample_scale);
    let mask = noise(n3_t, n3_s, offset_world_dir * stars.mask_scale );
    let blink_variance_noise = noise(n3_t, n3_s, offset_world_dir * stars.blink_variance_scale);

    // reduce star density with mask
    noise = noise * (1.0-smoothstep(stars.mask_threshold, 1.0, mask));

    let base_blink_speed = global_time * stars.blink_speed;
    // star blink in different speeds anywhere between 50% -> 150%
    let speed_variance = (1.0 + sin(blink_variance_noise) * 0.5);
    // 0: no blink, 1: full blink
    let blink = cos(base_blink_speed * speed_variance + blink_variance_noise);
    let blink_threshold = blink * stars.star_threshold_blink;
    var star_intensity = smoothstep(stars.star_threshold + blink_threshold, 1.0, noise);
    return star_intensity;
}

