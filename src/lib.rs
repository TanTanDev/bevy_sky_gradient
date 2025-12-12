pub mod ambient_driver;
pub mod assets;
pub mod aurora;
pub mod aurora_material;
pub mod bind_groups;
pub mod cycle;
pub mod gradient;
pub mod gradient_driver;
pub mod gradient_material;
pub mod noise;
pub mod plugin;
pub mod presets;
pub mod sky_material;
pub mod sky_texture;
pub mod sun;
pub mod utils;

pub mod prelude {
    pub use crate::ambient_driver::{AmbientDriverPlugin, AmbientSettings};
    pub use crate::aurora::{AuroraPlugin, AuroraSettings};
    pub use crate::cycle::{SkyCyclePlugin, SkyTime, SkyTimeSettings};
    pub use crate::gradient_driver::GradientDriverPlugin;
    pub use crate::noise::{NoisePlugin, NoiseSettings};
    pub use crate::plugin::{SkyPlugin, SkySettings, SkyboxMagnetTag};
    pub use crate::sun::{SunDriverPlugin, SunDriverTag, SunSettings};
}
