use bevy_inspector_egui::egui::Color32;

use crate::gradient::{SkyColorsBuilder, StopsColors};

pub const DEFAULT_SKY_COLORS_BUILDER: SkyColorsBuilder = SkyColorsBuilder {
    sunset_color: StopsColors {
        stop0: Color32::from_rgb(255, 70, 70),
        stop1: Color32::from_rgb(243, 84, 47),
        stop2: Color32::from_rgb(255, 242, 72),
        stop3: Color32::from_rgb(73, 177, 250),
    },
    sunrise_color: StopsColors {
        stop0: Color32::from_rgb(255, 70, 70),
        stop1: Color32::from_rgb(243, 84, 47),
        stop2: Color32::from_rgb(255, 242, 72),
        stop3: Color32::from_rgb(73, 177, 250),
    },
    day_low_color: StopsColors {
        stop0: Color32::from_rgb(157, 157, 248),
        stop1: Color32::from_rgb(205, 242, 255),
        stop2: Color32::from_rgb(182, 200, 254),
        stop3: Color32::from_rgb(224, 224, 255),
    },
    day_high_color: StopsColors {
        stop0: Color32::from_rgb(48, 48, 255),
        stop1: Color32::from_rgb(0, 226, 255),
        stop2: Color32::from_rgb(0, 170, 255),
        stop3: Color32::from_rgb(66, 195, 255),
    },
    night_low_color: StopsColors {
        stop0: Color32::from_rgb(0, 3, 40),
        stop1: Color32::from_rgb(47, 0, 93),
        stop2: Color32::from_rgb(0, 38, 97),
        stop3: Color32::from_rgb(74, 0, 89),
    },
    night_high_color: StopsColors {
        stop0: Color32::from_rgb(0, 0, 45),
        stop1: Color32::from_rgb(0, 32, 93),
        stop2: Color32::from_rgb(0, 0, 112),
        stop3: Color32::from_rgb(0, 0, 43),
    },
};
