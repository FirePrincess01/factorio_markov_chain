//! upcycle_loop
//!
//! Contains the user interface for a factorio upcycle loop

use std::{collections::BTreeMap, num};

use crate::ui::view_model::UpCyclingLoopViewModel;

use super::color_palette::{self, ColorPalette};
use egui::{
    pos2, vec2, Align2, Color32, FontId, Pos2, ProgressBar, Rect, Rounding, SelectableLabel, Sense,
    Stroke, Vec2,
};
use egui_plot::{self, Bar, BarChart, Legend, Plot};

struct Arrow {
    start: Pos2,
    end: Pos2,
}

impl Arrow {
    fn new2(start: Pos2, end: Pos2) -> Self {
        Self { start, end }
    }

    fn new(start: Pos2, size: Vec2) -> Self {
        Self {
            start,
            end: start + size,
        }
    }

    fn start(&self) -> Pos2 {
        self.start
    }

    fn end(&self) -> Pos2 {
        self.end
    }

    fn size(&self) -> Vec2 {
        self.end - self.start
    }
}

pub fn upcycle_loop(ctx: &egui::Context, ui: &mut egui::Ui, model: &mut UpCyclingLoopViewModel) {
    ui.add(egui::Separator::default());

    ui.style_mut().visuals.selection.bg_fill =
    Color32::from_hex(ColorPalette::COLOR_PINK_1).unwrap();

    ui.horizontal(|ui| {
        ui.heading("Production Facility ");
        ui.selectable_value(&mut model.factory_kind, 1, "Assembly machine 1");
        ui.selectable_value(&mut model.factory_kind, 2, "Assembly machine 2");
        ui.selectable_value(&mut model.factory_kind, 3, "Assembly machine 3");
        ui.selectable_value(&mut model.factory_kind, 4, "Electromagnetic plant");
        ui.selectable_value(&mut model.factory_kind, 5, "Foundry");
        ui.selectable_value(&mut model.factory_kind, 6, "Cryogenic plant");
        ui.selectable_value(&mut model.factory_kind, 7, "Bio chamber");
    });

    egui::Grid::new("some_unique_id2").show(ui, |ui| {
        productions_facilities(ctx, ui, model);
        // ui.add(egui::Separator::default());

        ui.end_row();

        ui.style_mut().visuals.selection.bg_fill =
        Color32::from_hex(ColorPalette::COLOR_BLUE_1).unwrap();
        recycler_facility(ctx, ui, model);
    });

    ui.add(egui::Separator::default());

    ui.heading("Upcycle Loop");

    ui.style_mut().visuals.selection.bg_fill =
    Color32::from_hex(ColorPalette::COLOR_PINK_1).unwrap();
    upcycle_loop_production(ctx, ui, model);
    upcycle_loop_diagram(ctx, ui);
    upcycle_loop_recycle(ctx, ui, model);

    ui.add(egui::Separator::default());

    conversion(ctx, ui, model);

    ui.add(egui::Separator::default());


}

fn conversion(ctx: &egui::Context, ui: &mut egui::Ui, model: &mut UpCyclingLoopViewModel) {

    egui::Grid::new("some_unique_id5")
    .min_col_width(110.0)
    .show(ui, |ui| {

        
        ui.label("Input Quality: ");
        print_percent(ui, model.factory_productivity_percent);

        ui.end_row();

        ui.label("Output Quality: ");
        print_percent(ui, model.factory_quality_percent);

        ui.end_row();

    });
}

fn upcycle_loop_production(ctx: &egui::Context, ui: &mut egui::Ui, model: &mut UpCyclingLoopViewModel) {
   ui.label("Production Facilities:");
    
    
    egui::Grid::new("some_unique_id3")
    .min_col_width(110.0)
    .show(ui, |ui| {

        
        ui.label("Production:                                        ");
        print_percent(ui, model.factory_productivity_percent);
        print_percent(ui, model.factory_productivity_percent);
        print_percent(ui, model.factory_productivity_percent);
        print_percent(ui, model.factory_productivity_percent);
        print_percent(ui, model.factory_productivity_percent);

        ui.end_row();

        ui.label("Quality: ");
        print_percent(ui, model.factory_quality_percent);
        print_percent(ui, model.factory_quality_percent);
        print_percent(ui, model.factory_quality_percent);
        print_percent(ui, model.factory_quality_percent);
        print_percent(ui, model.factory_quality_percent);

        ui.end_row();

        ui.label("Enabled: ");
        ui.checkbox(&mut model.factory_0_enabled, "enable");
        ui.checkbox(&mut model.factory_1_enabled, "enable");
        ui.checkbox(&mut model.factory_2_enabled, "enable");
        ui.checkbox(&mut model.factory_3_enabled, "enable");
        ui.checkbox(&mut model.factory_4_enabled, "enable");
        
        ui.end_row();

        ui.label("Load (Speed): ");
        print_value(ui, model.factory_0_load);
        print_value(ui, model.factory_1_load);
        print_value(ui, model.factory_2_load);
        print_value(ui, model.factory_3_load);
        print_value(ui, model.factory_4_load);

        ui.end_row();

    });
}

fn upcycle_loop_recycle(ctx: &egui::Context, ui: &mut egui::Ui, model: &mut UpCyclingLoopViewModel) {
   
   ui.label("Recycler:");
   
   egui::Grid::new("some_unique_id4")
    .show(ui, |ui| {

        
        ui.label("Production: ");
        print_percent(ui, model.recycler_productivity_percent);

        ui.end_row();

        ui.label("Quality: ");
        print_percent(ui, model.recycler_quality_percent);

        ui.end_row();

        ui.label("Load (Speed): ");
        print_value(ui, model.recycler_load);

        ui.end_row();

    });
}



pub fn productions_facilities(
    ctx: &egui::Context,
    ui: &mut egui::Ui,
    model: &mut UpCyclingLoopViewModel,
) {
    // ui.style_mut().visuals.selection.bg_fill =
    //     Color32::from_hex(ColorPalette::COLOR_PINK_1).unwrap();


    // egui::Grid::new("some_unique_id").show(ui, |ui| {
        ui.label("Nr Productivity Modules:     ");
        selectable_number_zero(
            ui,
            &mut model.factory_productivity_modules_amount,
            model.factory_module_slots,
        );

        ui.label("Nr Quality Modules:     ");
        selectable_number_zero(
            ui,
            &mut model.factory_quality_modules_amount,
            model.factory_module_slots,
        );
        ui.end_row();

        ui.label("Productivity Module Tier:    ");
        selectable_number(ui, &mut model.factory_productivity_modules_tier, 3);

        ui.label("Quality Module Tier:    ");
        selectable_number(ui, &mut model.factory_quality_modules_tier, 3);
        ui.end_row();

        ui.label("Productivity Module Quality: ");
        selectable_number(
            ui,
            &mut model.factory_productivity_modules_quality,
            model.max_researched_quality_level,
        );

        ui.label("Quality Module Quality: ");
        selectable_number(
            ui,
            &mut model.factory_quality_modules_quality,
            model.max_researched_quality_level,
        );
        ui.end_row();

        ui.label("Productivity:     ");
        show_percent(
            ui,
            model.factory_productivity_percent,
            model.factory_productivity_max_percent,
        );

        ui.label("Quality:     ");
        show_percent(
            ui,
            model.factory_quality_percent,
            model.factory_quality_max_percent,
        );
        ui.end_row();
    // });
}

fn recycler_facility(ctx: &egui::Context, ui: &mut egui::Ui, model: &mut UpCyclingLoopViewModel) {


    ui.heading("Recycler");
    ui.end_row();

    // egui::Grid::new("some_unique_id2").show(ui, |ui| {
        ui.label("Nr Productivity Modules:     ");
        selectable_number_zero(
            ui,
            &mut model.recycler_productivity_modules_amount,
            model.recycler_module_slots,
        );

        ui.label("Nr Quality Modules:     ");
        selectable_number_zero(
            ui,
            &mut model.recycler_quality_modules_amount,
            model.recycler_module_slots,
        );
        ui.end_row();

        ui.label("Productivity Module Tier:    ");
        selectable_number(ui, &mut model.recycler_productivity_modules_tier, 3);

        ui.label("Quality Module Tier:    ");
        selectable_number(ui, &mut model.recycler_quality_modules_tier, 3);
        ui.end_row();

        ui.label("Productivity Module Quality: ");
        selectable_number(
            ui,
            &mut model.recycler_productivity_modules_quality,
            model.max_researched_quality_level,
        );

        ui.label("Quality Module Quality: ");
        selectable_number(
            ui,
            &mut model.recycler_quality_modules_quality,
            model.max_researched_quality_level,
        );
        ui.end_row();

        ui.label("Productivity:     ");
        show_percent(
            ui,
            model.recycler_productivity_percent,
            model.recycler_productivity_max_percent,
        );

        ui.label("Quality:     ");
        show_percent(
            ui,
            model.recycler_quality_percent,
            model.recycler_quality_max_percent,
        );
        ui.end_row();
    // });
}

fn selectable_number(ui: &mut egui::Ui, value: &mut u32, max_value: u32) {
    ui.horizontal(|ui| {
        for i in 0..max_value {
            let num = i + 1;
            let num_as_string = format!("  {}  ", num);
            ui.selectable_value(value, num, num_as_string);
        }
    });
}

fn selectable_number_zero(ui: &mut egui::Ui, value: &mut u32, max_value: u32) {
    ui.horizontal(|ui| {
        for i in 0..max_value + 1 {
            let num = i;
            let num_as_string = format!("  {}  ", num);
            ui.selectable_value(value, num, num_as_string);
        }
    });
}

fn print_value(ui: &mut egui::Ui, value: f32) {
    let num_as_string = format!("{:>3}", value);
    ui.label(num_as_string);
}

fn print_percent(ui: &mut egui::Ui, value: f32) {
    let num_as_string = format!("{:>3} %", value);
    ui.label(num_as_string);
}

fn show_percent(ui: &mut egui::Ui, value: f32, max_value: f32) {
    let progress = value / max_value;
    let num_as_string = format!("{} %", value);
    let progress_bar = ProgressBar::new(progress)
        .show_percentage()
        .text(num_as_string);
    ui.add(progress_bar);
}

fn upcycle_loop_diagram(ctx: &egui::Context, ui: &mut egui::Ui) {
    let pink = ColorPalette::from_pink();
    let blue = ColorPalette::from_blue();

    let width = 800.0;
    let hight = 200.0;
    let (rect, _response) = ui.allocate_at_least(vec2(width, hight), Sense::hover());
    let x0 = rect.min.x;
    let y0 = rect.min.y;

    let factory_width = 100.0;
    let factory_hight = 100.0;

    let recycler_width = 125.0;
    let recycler_height = 75.0;

    let border = 10.0;
    let y_middle = y0 + hight / 2.0;

    let separation = 20.0;

    let arrow_length = 50.0;

    let rect_recycler = Rect::from_center_size(
        pos2(x0 + border + recycler_width / 2.0, y_middle),
        vec2(recycler_width, recycler_height),
    );

    let rect_factory0 = Rect::from_center_size(
        pos2(
            rect_recycler.right_center().x + separation + factory_width / 2.0,
            y_middle,
        ),
        vec2(factory_width, recycler_height),
    );

    let rect_factory1 = Rect::from_center_size(
        pos2(
            rect_factory0.right_center().x + separation + factory_width / 2.0,
            y_middle,
        ),
        vec2(factory_width, recycler_height),
    );

    let rect_factory2 = Rect::from_center_size(
        pos2(
            rect_factory1.right_center().x + separation + factory_width / 2.0,
            y_middle,
        ),
        vec2(factory_width, recycler_height),
    );

    let rect_factory3 = Rect::from_center_size(
        pos2(
            rect_factory2.right_center().x + separation + factory_width / 2.0,
            y_middle,
        ),
        vec2(factory_width, recycler_height),
    );

    let rect_factory4 = Rect::from_center_size(
        pos2(
            rect_factory3.right_center().x + separation + factory_width / 2.0,
            y_middle,
        ),
        vec2(factory_width, recycler_height),
    );

    // arrow
    let arrow_bottom_recycler = Arrow::new(
        pos2(
            rect_recycler.center_bottom().x,
            rect_recycler.center_bottom().y,
        ),
        vec2(0.0, arrow_length),
    );

    let arrow_bottom_0 = Arrow::new(
        pos2(
            rect_factory0.center_bottom().x,
            rect_factory0.center_bottom().y + arrow_length,
        ),
        vec2(0.0, -arrow_length),
    );

    let arrow_bottom_1 = Arrow::new(
        pos2(
            rect_factory1.center_bottom().x,
            rect_factory1.center_bottom().y + arrow_length,
        ),
        vec2(0.0, -arrow_length),
    );

    let arrow_bottom_2 = Arrow::new(
        pos2(
            rect_factory2.center_bottom().x,
            rect_factory2.center_bottom().y + arrow_length,
        ),
        vec2(0.0, -arrow_length),
    );

    let arrow_bottom_3 = Arrow::new(
        pos2(
            rect_factory3.center_bottom().x,
            rect_factory3.center_bottom().y + arrow_length,
        ),
        vec2(0.0, -arrow_length),
    );

    let arrow_bottom_4 = Arrow::new(
        pos2(
            rect_factory4.center_bottom().x,
            rect_factory4.center_bottom().y + arrow_length,
        ),
        vec2(0.0, -arrow_length),
    );

    // arrow bottom horizontal
    let arrow_bottom_horizontal_0 =
        Arrow::new2(arrow_bottom_recycler.end(), arrow_bottom_0.start());
    let arrow_bottom_horizontal_1 = Arrow::new2(arrow_bottom_0.start(), arrow_bottom_1.start());
    let arrow_bottom_horizontal_2 = Arrow::new2(arrow_bottom_1.start(), arrow_bottom_2.start());
    let arrow_bottom_horizontal_3 = Arrow::new2(arrow_bottom_2.start(), arrow_bottom_3.start());
    let arrow_bottom_horizontal_4 = Arrow::new2(arrow_bottom_3.start(), arrow_bottom_4.start());

    // arrow top
    let arrow_top_recycler = Arrow::new(
        pos2(
            rect_recycler.center_top().x,
            rect_recycler.center_top().y - arrow_length,
        ),
        vec2(0.0, arrow_length),
    );

    let arrow_top_0 = Arrow::new(
        pos2(rect_factory0.center_top().x, rect_factory0.center_top().y),
        vec2(0.0, -arrow_length),
    );

    let arrow_top_1 = Arrow::new(
        pos2(rect_factory1.center_top().x, rect_factory1.center_top().y),
        vec2(0.0, -arrow_length),
    );

    let arrow_top_2 = Arrow::new(
        pos2(rect_factory2.center_top().x, rect_factory2.center_top().y),
        vec2(0.0, -arrow_length),
    );

    let arrow_top_3 = Arrow::new(
        pos2(rect_factory3.center_top().x, rect_factory3.center_top().y),
        vec2(0.0, -arrow_length),
    );

    let arrow_top_4 = Arrow::new(
        pos2(rect_factory4.center_top().x, rect_factory4.center_top().y),
        vec2(0.0, -arrow_length),
    );

    // arrow top horizontal
    let arrow_top_horizontal_0 = Arrow::new2(arrow_top_recycler.start(), arrow_top_0.end());
    let arrow_top_horizontal_1 = Arrow::new2(arrow_top_0.end(), arrow_top_1.end());
    let arrow_top_horizontal_2 = Arrow::new2(arrow_top_1.end(), arrow_top_2.end());
    let arrow_top_horizontal_3 = Arrow::new2(arrow_top_2.end(), arrow_top_3.end());
    let arrow_top_horizontal_4 = Arrow::new2(arrow_top_3.end(), arrow_top_4.end());

    let painter = ui.painter();
    let rounding = Rounding::from(10.0);
    let stroke = Stroke::NONE;

    // paint rectangles
    // painter.rect(rect, rounding, pink.colors[0], stroke);
    painter.rect(rect_recycler, rounding, blue.colors[1], stroke);
    painter.rect(rect_factory0, rounding, pink.colors[1], stroke);
    painter.rect(rect_factory1, rounding, pink.colors[2], stroke);
    painter.rect(rect_factory2, rounding, pink.colors[3], stroke);
    painter.rect(rect_factory3, rounding, pink.colors[4], stroke);
    painter.rect(rect_factory4, rounding, pink.colors[5], stroke);

    // paint arrows bottom up
    let stroke = Stroke::new(2.0, blue.colors[3]);
    painter.arrow(
        arrow_bottom_recycler.start(),
        arrow_bottom_recycler.size(),
        stroke,
    );
    painter.arrow(arrow_bottom_0.start(), arrow_bottom_0.size(), stroke);
    painter.arrow(arrow_bottom_1.start(), arrow_bottom_1.size(), stroke);
    painter.arrow(arrow_bottom_2.start(), arrow_bottom_2.size(), stroke);
    painter.arrow(arrow_bottom_3.start(), arrow_bottom_3.size(), stroke);
    painter.arrow(arrow_bottom_4.start(), arrow_bottom_4.size(), stroke);

    painter.line_segment(
        [
            arrow_bottom_horizontal_0.start(),
            arrow_bottom_horizontal_0.end(),
        ],
        stroke,
    );
    painter.line_segment(
        [
            arrow_bottom_horizontal_1.start(),
            arrow_bottom_horizontal_1.end(),
        ],
        stroke,
    );
    painter.line_segment(
        [
            arrow_bottom_horizontal_2.start(),
            arrow_bottom_horizontal_2.end(),
        ],
        stroke,
    );
    painter.line_segment(
        [
            arrow_bottom_horizontal_3.start(),
            arrow_bottom_horizontal_3.end(),
        ],
        stroke,
    );
    painter.line_segment(
        [
            arrow_bottom_horizontal_4.start(),
            arrow_bottom_horizontal_4.end(),
        ],
        stroke,
    );

    let stroke = Stroke::new(2.0, pink.colors[5]);
    painter.arrow(
        arrow_top_recycler.start(),
        arrow_top_recycler.size(),
        stroke,
    );
    let stroke = Stroke::new(2.0, pink.colors[1]);
    painter.arrow(arrow_top_0.start(), arrow_top_0.size(), stroke);
    let stroke = Stroke::new(2.0, pink.colors[2]);
    painter.arrow(arrow_top_1.start(), arrow_top_1.size(), stroke);
    let stroke = Stroke::new(2.0, pink.colors[3]);
    painter.arrow(arrow_top_2.start(), arrow_top_2.size(), stroke);
    let stroke = Stroke::new(2.0, pink.colors[4]);
    painter.arrow(arrow_top_3.start(), arrow_top_3.size(), stroke);
    let stroke = Stroke::new(2.0, pink.colors[5]);
    painter.arrow(arrow_top_4.start(), arrow_top_4.size(), stroke);

    let stroke = Stroke::new(2.0, pink.colors[5]);
    painter.line_segment(
        [arrow_top_horizontal_0.start(), arrow_top_horizontal_0.end()],
        stroke,
    );
    painter.line_segment(
        [arrow_top_horizontal_1.start(), arrow_top_horizontal_1.end()],
        stroke,
    );
    painter.line_segment(
        [arrow_top_horizontal_2.start(), arrow_top_horizontal_2.end()],
        stroke,
    );
    painter.line_segment(
        [arrow_top_horizontal_3.start(), arrow_top_horizontal_3.end()],
        stroke,
    );
    painter.line_segment(
        [arrow_top_horizontal_4.start(), arrow_top_horizontal_4.end()],
        stroke,
    );

    // text
    // painter.text(
    //     rect_factory0.center_top(),
    //     Align2::CENTER_TOP,
    //     "speed: 1.72",
    //     FontId::default(),
    //     blue.colors[0],
    // );
}
