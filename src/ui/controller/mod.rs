//! # controller
//!
//! Contains the user interface code of this application
//! Tries to separate the functionality similar to the the Model View Viewmodel pattern

use crate::ui::view;

use super::view::bar_chart;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Controller {

}

impl Controller {
    pub fn new() -> Self {
        Self {  }
    }
    
    pub fn update(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        view::heading(ctx, ui);
        view::bar_chart(ctx, ui);
    }
}


