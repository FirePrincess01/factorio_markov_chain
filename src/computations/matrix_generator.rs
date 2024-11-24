use nalgebra::{Matrix5, Vector5};

fn squishing_matrix(min_q: usize, max_q: usize) -> Matrix5<f64> {
    let mut squish_squish = Matrix5::zeros();
    if min_q > max_q {
        return squish_squish;
    }
    for i in 0..5 {
        squish_squish[(std::cmp::min(i + min_q - 1, max_q - 1), i)] = 1.;
    }
    return squish_squish;
}

pub fn crafting_matrix(quality: f64, productivity: f64, max_q: usize) -> Matrix5<f64> {
    let q = quality;
    let col_q = Vector5::new(1. - q, 0.9 * q, 0.09 * q, 0.009 * q, 0.001 * q);

    let mut craft_craft = Matrix5::zeros();
    for i in 0..5 {
        let squish_squish = squishing_matrix(i + 1, max_q);
        // let coloum = squish_squish * col_q;
        craft_craft.set_column(i, &(squish_squish * col_q));
    }
    craft_craft *= 1. + productivity;
    return craft_craft;
}

pub fn recycling_matrix(quality: f64, max_q: usize) -> Matrix5<f64> {
    return crafting_matrix(quality, -0.75, max_q);
}