//! # view
//!
//! Contains the user interface code of this application
//! Tries to separate the functionality similar to the the Model View Viewmodel pattern

mod bar_chart;
mod color_palette;

pub use bar_chart::*;
use egui::{Color32, RichText};

/// Creates the top heading of the application
pub fn heading(ctx: &egui::Context, ui: &mut egui::Ui) 
{
    let light_blue = Color32::from_rgb(91, 206, 250);
    let pink = Color32::from_rgb(245, 169, 184);
    let white = Color32::from_rgb(255, 255, 255);

    let cecilia_sandbox = RichText::new("CeciliaSandbox").size(20.0).color(pink);
    let and = RichText::new("&").size(20.0).color(white);
    let fire_princess_01 = RichText::new("FirePrincess01").size(20.0).color(light_blue);

    let name = RichText::new("Factorio Markov Chain Quality Calculator").size(30.0);

    ui.horizontal(|ui| {
        ui.label(cecilia_sandbox);
        ui.label(and);
        ui.label(fire_princess_01);
    });

    ui.label(name);

}