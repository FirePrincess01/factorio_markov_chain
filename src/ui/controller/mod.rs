//! # controller
//!
//! Contains the user interface code of this application
//! Tries to separate the functionality similar to the the Model View Viewmodel pattern

use crate::ui::view;

use super::view::bar_chart;
use super::view::upcycle_loop;
use super::view_model::UpCyclingLoopViewModel;

// #[derive(serde::Deserialize, serde::Serialize)]
pub struct Controller {
    up_cycling_loop_model: UpCyclingLoopViewModel,
}

impl Controller {
    pub fn new() -> Self {
        let  up_cycling_loop_model: UpCyclingLoopViewModel = UpCyclingLoopViewModel::new();

        Self { up_cycling_loop_model}
    }
    
    pub fn update(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        view::heading(ctx, ui);
        view::upcycle_loop(ctx, ui, &mut self.up_cycling_loop_model);
        view::bar_chart(ctx, ui);

        
    }
}


