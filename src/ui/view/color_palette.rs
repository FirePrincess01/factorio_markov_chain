//! # color palette
//!
//! Contains color palettes for graphs and charts

use egui::Color32;

pub struct ColorPalette {
    pub colors: [Color32; 8],
}

impl ColorPalette {
    // pinkish color palette
    pub const COLOR_PINK_0: &str = "#2D0B68";
    pub const COLOR_PINK_1: &str = "#7B3DBB";
    pub const COLOR_PINK_2: &str = "#AD4DC8";
    pub const COLOR_PINK_3: &str = "#D94FAE";
    pub const COLOR_PINK_4: &str = "#E9568A";
    pub const COLOR_PINK_5: &str = "#E8736B";
    pub const COLOR_PINK_6: &str = "#EFA490";
    pub const COLOR_PINK_7: &str = "#FCE9E6";

    // blueish color palette
    pub const COLOR_BLUE_0: &str = "#1A2958";
    pub const COLOR_BLUE_1: &str = "#1F6B9C";
    pub const COLOR_BLUE_2: &str = "#24A8DA";
    pub const COLOR_BLUE_3: &str = "#27C7E3";
    pub const COLOR_BLUE_4: &str = "#29D2D2";
    pub const COLOR_BLUE_5: &str = "#2CE3BD";
    pub const COLOR_BLUE_6: &str = "#6FEFD2";
    pub const COLOR_BLUE_7: &str = "#C6F9EE";

    pub fn from_pink() -> Self {
        let colors: [Color32; 8] = [
            Color32::from_hex(Self::COLOR_PINK_0).unwrap(),
            Color32::from_hex(Self::COLOR_PINK_1).unwrap(),
            Color32::from_hex(Self::COLOR_PINK_2).unwrap(),
            Color32::from_hex(Self::COLOR_PINK_3).unwrap(),
            Color32::from_hex(Self::COLOR_PINK_4).unwrap(),
            Color32::from_hex(Self::COLOR_PINK_5).unwrap(),
            Color32::from_hex(Self::COLOR_PINK_6).unwrap(),
            Color32::from_hex(Self::COLOR_PINK_7).unwrap(),
        ];

        Self { colors }
    }

    pub fn from_blue() -> Self {
        let colors: [Color32; 8] = [
            Color32::from_hex(Self::COLOR_BLUE_0).unwrap(),
            Color32::from_hex(Self::COLOR_BLUE_1).unwrap(),
            Color32::from_hex(Self::COLOR_BLUE_2).unwrap(),
            Color32::from_hex(Self::COLOR_BLUE_3).unwrap(),
            Color32::from_hex(Self::COLOR_BLUE_4).unwrap(),
            Color32::from_hex(Self::COLOR_BLUE_5).unwrap(),
            Color32::from_hex(Self::COLOR_BLUE_6).unwrap(),
            Color32::from_hex(Self::COLOR_BLUE_7).unwrap(),
        ];

        Self { colors }
    }
}



