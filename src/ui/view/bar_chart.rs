//! # bar_chart
//!
//! Contains the user interface code of this application
//! Tries to separate the functionality similar to the the Model View Viewmodel pattern

use egui::Color32;
use egui_plot::{self, Bar, BarChart, Legend, Plot};
use super::color_palette;

pub fn bar_chart(ctx: &egui::Context, ui: &mut egui::Ui)
{   
    let color0 = Color32::from_hex(color_palette::COLOR_PINK_0).unwrap();
    let color1 = Color32::from_hex(color_palette::COLOR_PINK_1).unwrap();
    let color2 = Color32::from_hex(color_palette::COLOR_PINK_2).unwrap();
    let color3 = Color32::from_hex(color_palette::COLOR_PINK_3).unwrap();
    let color4 = Color32::from_hex(color_palette::COLOR_PINK_4).unwrap();
    let color5 = Color32::from_hex(color_palette::COLOR_PINK_5).unwrap();
    let color6 = Color32::from_hex(color_palette::COLOR_PINK_6).unwrap();
    let color7 = Color32::from_hex(color_palette::COLOR_PINK_7).unwrap();


    let bars1 = vec![
        Bar::new(1.0, 5.0).fill(color0),
        Bar::new(2.0, 6.0).fill(color1),
        Bar::new(3.0, 7.0).fill(color2),
        Bar::new(4.0, 8.0).fill(color3),
        Bar::new(5.0, 9.0).fill(color4),
        Bar::new(6.0, 10.0).fill(color5),
        Bar::new(7.0, 11.0).fill(color6),
        Bar::new(8.0, 12.0).fill(color7),
    ];

    let color0 = Color32::from_hex(color_palette::COLOR_BLUE_0).unwrap();
    let color1 = Color32::from_hex(color_palette::COLOR_BLUE_1).unwrap();
    let color2 = Color32::from_hex(color_palette::COLOR_BLUE_2).unwrap();
    let color3 = Color32::from_hex(color_palette::COLOR_BLUE_3).unwrap();
    let color4 = Color32::from_hex(color_palette::COLOR_BLUE_4).unwrap();
    let color5 = Color32::from_hex(color_palette::COLOR_BLUE_5).unwrap();
    let color6 = Color32::from_hex(color_palette::COLOR_BLUE_6).unwrap();
    let color7 = Color32::from_hex(color_palette::COLOR_BLUE_7).unwrap();


    let bars2 = vec![
        Bar::new(1.0, 5.0).fill(color0),
        Bar::new(2.0, 6.0).fill(color1),
        Bar::new(3.0, 7.0).fill(color2),
        Bar::new(4.0, 8.0).fill(color3),
        Bar::new(5.0, 9.0).fill(color4),
        Bar::new(6.0, 10.0).fill(color5),
        Bar::new(7.0, 11.0).fill(color6),
        Bar::new(8.0, 12.0).fill(color7),
    ];

    // ui.horizontal(|ui| {
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

        // Plot::new("demo chart")
        // .legend(Legend::default())
        // .show(ui, |plot_ui| {
        //     plot_ui.add(chart2);
        // });
// });







}