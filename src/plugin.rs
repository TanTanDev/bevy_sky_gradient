use bevy::prelude::*;

use crate::gradient_material::SkyGradientMaterial;
#[derive(Default)]
pub struct SkyGradientPlugin;

impl Plugin for SkyGradientPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<SkyGradientMaterial>::default());
    }
}
