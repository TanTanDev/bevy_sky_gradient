// just a helper to make noise calls look cleaner
// scale: size of the texture scale, used to sample same positions, regardless of texture size
fn noise(t: texture_3d<f32>, s: sampler, pos: vec3f) -> f32 {
    return textureSample(t, s, pos).r;
}
