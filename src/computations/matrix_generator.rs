//! matrix_generator
//!
//! utility functions to generate matrices needed for recycling loops.

use nalgebra::{Matrix5, Vector5};

/// Generates a matrix , which moves vector entries down by min_q - 1 and sums all entries beyond max_q into the entry with index max_q.
fn squishing_matrix(min_q: usize, max_q: usize) -> Matrix5<f64> {
    let mut squish_squish = Matrix5::zeros();
    if min_q > max_q {
        return squish_squish;
    }
    for i in 0..5 {
        squish_squish[(std::cmp::min(i + min_q - 1, max_q - 1), i)] = 1.;
    }
    squish_squish
}

/// Creates a transition matrix mapping an ingredient distribution to a product distribution
/// It features productivity and quality given as a ratio of 1
/// max_q states the maximum researched quality
pub fn crafting_matrix(quality: f64, productivity: f64, max_q: usize) -> Matrix5<f64> {
    let q = quality;
    let col_q = Vector5::new(1. - q, 0.9 * q, 0.09 * q, 0.009 * q, 0.001 * q);

    let mut craft_craft = Matrix5::zeros();
    for i in 0..5 {
        let squish_squish = squishing_matrix(i + 1, max_q);
        craft_craft.set_column(i, &(squish_squish * col_q));
    }
    craft_craft *= 1. + productivity;
    craft_craft
}

/// Creates a transition matrix mapping a product distribution to an ingredient distribution
/// It features quality given as a ratio of 1
/// max_q states the maximum researched quality
pub fn recycling_matrix(quality: f64, max_q: usize) -> Matrix5<f64> {
    crafting_matrix(quality, -0.75, max_q)
}

/// returns a distribution of only common items.
pub fn common_only_distribution() -> Vector5<f64> {
    Vector5::<f64>::new(1., 0., 0., 0., 0.)
}
