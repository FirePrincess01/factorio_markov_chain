//! recycling_loop
//!
//! Represents a Quality Grinding loop, which involves repeatedly crafting and recycling a recipe until a desired quality is reached.

use nalgebra::{RowVector5, SMatrix, Vector5};

use super::matrix_generator::{crafting_matrix, recycling_matrix};
type Matrix10 = SMatrix<f64, 10, 10>;
type Vector10 = SMatrix<f64, 10, 1>;

pub struct RecyclingLoop {
    pub markov_loop: Matrix10,
    pub limit_loop: Matrix10,
    target_index: usize,
}

impl RecyclingLoop {
    pub fn new(
        recycling_quality: f64,
        crafting_quality: f64,
        productivity: f64,
        stop_at_product: bool,
        max_quality: usize,
    ) -> Self {
        let mut markov_loop = Matrix10::zeros();
        let recycle = recycling_matrix(recycling_quality, max_quality);
        let craft = crafting_matrix(crafting_quality, productivity, max_quality);

        {
            let mut recycle_sub = markov_loop.view_mut((0, 5), (5, 5));
            for i in 0..5 {
                recycle_sub.set_column(i, &recycle.column(i));
            }
        }

        {
            let mut crafting_sub = markov_loop.view_mut((5, 0), (5, 5));
            for i in 0..5 {
                crafting_sub.set_column(i, &craft.column(i));
            }
        }

        // This could be iterated over several outputs.
        let target_index = if stop_at_product { 9 } else { 4 };
        markov_loop.fill_column(target_index, 0.);
        markov_loop[(target_index, target_index)] = 1.;

        let mut limit_loop = Matrix10::zeros();
        limit_loop.copy_from(&markov_loop);

        // limit_loop = limit_loop^2^2^(6-2) = 65536
        for _i in 0..6 {
            limit_loop = limit_loop * limit_loop;
        }

        Self {
            markov_loop,
            limit_loop,
            target_index,
        }
    }

    /// Takes a distribution of inputs,
    /// which are either ingredients or the product of the recipe and
    /// upcycles them until only the target product remains.
    /// Returns the ratio of inputs converted to the target.
    pub fn upcycle_input(&self, input: Vector5<f64>, is_ingredient: bool) -> f64 {
        let shift_index = if is_ingredient { 0 } else { 5 };
        let mut input_expanded = Vector10::zeros();
        for i in 0..5 {
            input_expanded[(i + shift_index, 0)] = input[(i, 0)];
        }

        let terminal_distribution = self.limit_loop * input_expanded;
        return terminal_distribution[(self.target_index, 0)];
    }

    /// Calculates the needed crafting speed for each step for a given input.
    /// input is a quality distribution over the 5 quality levels.
    /// is_ingredient determines wether ingredients or the product is fed to the loop
    /// A 1 in the input is the rate the input is produced/consumed by one crafting machine of speed 1
    pub fn calculate_load(&self, input: Vector5<f64>, is_ingredient: bool) -> (Vector5<f64>, f64) {
        let shift_index = if is_ingredient { 0 } else { 5 };
        const ERROR_TOLERANCE: f64 = 0.0001;
        // recycler reverses recipies at 16 times the speed.
        const RECYCLING_TIME: f64 = 1. / 16.;
        const MAX_LOOPS: usize = 100;
        let input_quantity = input.sum();

        let mut input_expanded = Vector10::zeros();
        for i in 0..5 {
            input_expanded[(i + shift_index, 0)] = input[(i, 0)];
        }

        let mut crafting_load = Vector5::<f64>::zeros();
        // Dont recycle the legendary product
        let to_recycle =
            RowVector5::<f64>::new(1., 1., 1., 1., if self.target_index == 9 { 0. } else { 1. });
        let mut recycling_load = 0.;
        for _ in 1..MAX_LOOPS {
            crafting_load += input_expanded.view_mut((0, 0), (5, 1));
            recycling_load += (to_recycle * input_expanded.view_mut((5, 0), (5, 1))).sum();

            input_expanded = self.markov_loop * input_expanded;
            let remainder =
                (input_expanded.sum() - input_expanded[(self.target_index, 0)]) / input_quantity;
            if remainder < ERROR_TOLERANCE {
                break;
            }
        }

        // Dont craft, if we keep legendary ingredients.
        if self.target_index == 4 {
            crafting_load[(4, 0)] = 0.;
        }

        recycling_load *= RECYCLING_TIME;

        return (crafting_load, recycling_load);
    }
}
