use nalgebra::{SMatrix, U10};

use super::matrix_generator::{crafting_matrix, recycling_matrix};
type Matrix10 = SMatrix<f64, 10, 10>;

pub struct RecyclingLoop {
    pub markov_loop: Matrix10,
    pub limit_loop: Matrix10,
}

impl RecyclingLoop {
    pub fn new(
        recycling_quality: f64,
        crafting_quality: f64,
        productivity: f64,
        max_quality: usize,
        stop_at_product: bool,
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

        // This could be looped
        let stop_index = if stop_at_product { 9 } else { 4 };
        markov_loop.fill_column(stop_index, 0.);
        markov_loop[(stop_index, stop_index)] = 1.;

        let mut limit_loop = Matrix10::zeros();
        limit_loop.copy_from(&markov_loop);

        // limit_loop = limit_loop^256
        for _i in 0..5 {
            limit_loop = limit_loop * limit_loop;
        }

        Self {
            markov_loop,
            limit_loop,
        }
    }
}
