//! # bar_chart
//!
//! Contains the user interface code of this application
//! Tries to separate the functionality similar to the the Model View Viewmodel pattern

use super::color_palette::ColorPalette;
use egui::Color32;
use egui_plot::{self, Bar, BarChart, Legend, Plot};

pub fn bar_chart(ctx: &egui::Context, ui: &mut egui::Ui) {
    let pink = ColorPalette::from_pink();
    let blue = ColorPalette::from_blue();

    let bars1 = vec![
        Bar::new(1.0,  5.0).fill(pink.colors[0]),
        Bar::new(2.0,  6.0).fill(pink.colors[1]),
        Bar::new(3.0,  7.0).fill(pink.colors[2]),
        Bar::new(4.0,  8.0).fill(pink.colors[3]),
        Bar::new(5.0,  9.0).fill(pink.colors[4]),
        Bar::new(6.0, 10.0).fill(pink.colors[5]),
        Bar::new(7.0, 11.0).fill(pink.colors[6]),
        Bar::new(8.0, 12.0).fill(pink.colors[7]),
    ];

    let bars2 = vec![
        Bar::new(1.0,  5.0).fill(blue.colors[0]),
        Bar::new(2.0,  6.0).fill(blue.colors[1]),
        Bar::new(3.0,  7.0).fill(blue.colors[2]),
        Bar::new(4.0,  8.0).fill(blue.colors[3]),
        Bar::new(5.0,  9.0).fill(blue.colors[4]),
        Bar::new(6.0, 10.0).fill(blue.colors[5]),
        Bar::new(7.0, 11.0).fill(blue.colors[6]),
        Bar::new(8.0, 12.0).fill(blue.colors[7]),
    ];

    let chart1 = BarChart::new(bars1);
    let chart2 = BarChart::new(bars2);

    Plot::new("demo chart")
        .allow_zoom(false)
        .allow_drag(false)
        .allow_scroll(false)
        .legend(Legend::default())
        .height(300.0)
        .width(400.0)
        .show(ui, |plot_ui| {
            plot_ui.add(chart1);
            plot_ui.add(chart2);
        });
}
