//! # view_model
//!
//! Contains the user interface code of this application
//! Tries to separate the functionality similar to the the Model View Viewmodel pattern

pub struct UpCyclingLoopViewModel {
    pub factory_0_enabled: bool,
    pub factory_1_enabled: bool,
    pub factory_2_enabled: bool,
    pub factory_3_enabled: bool,
    pub factory_4_enabled: bool,

    pub product_0_output_enabled: bool,
    pub product_1_output_enabled: bool,
    pub product_2_output_enabled: bool,
    pub product_3_output_enabled: bool,
    pub product_4_output_enabled: bool,

    pub ingredient_0_output_enabled: bool,
    pub ingredient_1_output_enabled: bool,
    pub ingredient_2_output_enabled: bool,
    pub ingredient_3_output_enabled: bool,
    pub ingredient_4_output_enabled: bool,

    pub product_0_input_enabled: bool,
    pub product_1_input_enabled: bool,
    pub product_2_input_enabled: bool,
    pub product_3_input_enabled: bool,
    pub product_4_input_enabled: bool,

    pub ingredient_0_input_enabled: bool,
    pub ingredient_1_input_enabled: bool,
    pub ingredient_2_input_enabled: bool,
    pub ingredient_3_input_enabled: bool,
    pub ingredient_4_input_enabled: bool,

    pub product_0_output_percent: f32,
    pub product_1_output_percent: f32,
    pub product_2_output_percent: f32,
    pub product_3_output_percent: f32,
    pub product_4_output_percent: f32,

    pub ingredient_0_output_percent: f32,
    pub ingredient_1_output_percent: f32,
    pub ingredient_2_output_percent: f32,
    pub ingredient_3_output_percent: f32,
    pub ingredient_4_output_percent: f32,

    pub recycler_load: f32,
    pub factory_0_load: f32,
    pub factory_1_load: f32,
    pub factory_2_load: f32,
    pub factory_3_load: f32,
    pub factory_4_load: f32,

    pub product_0_input_percent: f32,
    pub product_1_input_percent: f32,
    pub product_2_input_percent: f32,
    pub product_3_input_percent: f32,
    pub product_4_input_percent: f32,

    pub ingredient_0_input_percent: f32,
    pub ingredient_1_input_percent: f32,
    pub ingredient_2_input_percent: f32,
    pub ingredient_3_input_percent: f32,
    pub ingredient_4_input_percent: f32,

    pub factory_productivity_percent: f32,
    pub factory_productivity_modules_amount: u32,
    pub factory_productivity_modules_tier: u32,
    pub factory_productivity_modules_quality: u32,

    pub recycler_productivity_percent: f32,
    pub recycler_productivity_modules_amount: u32,
    pub recycler_productivity_modules_tier: u32,
    pub recycler_productivity_modules_quality: u32,

    pub factory_quality_percent: f32,
    pub factory_quality_modules_amount: u32,
    pub factory_quality_modules_tier: u32,
    pub factory_quality_modules_quality: u32,

    pub recycler_quality_percent: f32,
    pub recycler_quality_modules_amount: u32,
    pub recycler_quality_modules_tier: u32,
    pub recycler_quality_modules_quality: u32,

    pub factory_kind: u32,
    pub factory_module_slots: u32,
    pub recycler_module_slots: u32,
    pub max_researched_quality_level: u32,
    pub factory_productivity_max_percent: f32,
    pub factory_quality_max_percent: f32,
    pub recycler_productivity_max_percent: f32,
    pub recycler_quality_max_percent: f32,
}

impl UpCyclingLoopViewModel {
    pub fn new() -> Self {
        Self {
            factory_0_enabled: true,
            factory_1_enabled: true,
            factory_2_enabled: true,
            factory_3_enabled: true,
            factory_4_enabled: true,

            product_0_output_enabled: false,
            product_1_output_enabled: false,
            product_2_output_enabled: false,
            product_3_output_enabled: false,
            product_4_output_enabled: true,

            ingredient_0_output_enabled: false,
            ingredient_1_output_enabled: false,
            ingredient_2_output_enabled: false,
            ingredient_3_output_enabled: false,
            ingredient_4_output_enabled: false,

            product_0_input_enabled: true,
            product_1_input_enabled: false,
            product_2_input_enabled: false,
            product_3_input_enabled: false,
            product_4_input_enabled: false,

            ingredient_0_input_enabled: false,
            ingredient_1_input_enabled: false,
            ingredient_2_input_enabled: false,
            ingredient_3_input_enabled: false,
            ingredient_4_input_enabled: false,

            product_0_output_percent: 0.0,
            product_1_output_percent: 0.0,
            product_2_output_percent: 0.0,
            product_3_output_percent: 0.0,
            product_4_output_percent: 0.0,

            ingredient_0_output_percent: 0.0,
            ingredient_1_output_percent: 0.0,
            ingredient_2_output_percent: 0.0,
            ingredient_3_output_percent: 0.0,
            ingredient_4_output_percent: 0.0,

            recycler_load: 0.0,
            factory_0_load: 0.0,
            factory_1_load: 0.0,
            factory_2_load: 0.0,
            factory_3_load: 0.0,
            factory_4_load: 0.0,

            product_0_input_percent: 100.0,
            product_1_input_percent: 0.0,
            product_2_input_percent: 0.0,
            product_3_input_percent: 0.0,
            product_4_input_percent: 0.0,

            ingredient_0_input_percent: 0.0,
            ingredient_1_input_percent: 0.0,
            ingredient_2_input_percent: 0.0,
            ingredient_3_input_percent: 0.0,
            ingredient_4_input_percent: 0.0,

            factory_productivity_percent: 150.0,
            factory_productivity_modules_amount: 3,
            factory_productivity_modules_tier: 3,
            factory_productivity_modules_quality: 5,

            factory_quality_percent: 25.0,
            factory_quality_modules_amount: 5,
            factory_quality_modules_tier: 3,
            factory_quality_modules_quality: 5,

            recycler_productivity_percent: 150.0,
            recycler_productivity_modules_amount: 0,
            recycler_productivity_modules_tier: 3,
            recycler_productivity_modules_quality: 5,
            
            recycler_quality_percent: 25.0,
            recycler_quality_modules_amount: 4,
            recycler_quality_modules_tier: 3,
            recycler_quality_modules_quality: 5,
            
            factory_kind: 6,
            factory_module_slots: 8,
            recycler_module_slots: 4,
            max_researched_quality_level: 5,
            factory_productivity_max_percent: 300.0,
            factory_quality_max_percent: 50.0,
            recycler_productivity_max_percent: 100.0,
            recycler_quality_max_percent: 50.0,
        }
    }
}

