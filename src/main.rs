mod computations;

fn main() {
    println!("Hello, world!");
    //let test = computations::matrix_generator::squishing_matrix(2,4);
    //let test = computations::matrix_generator::crafting_matrix(0.25,0., 5);
    //let test = computations::matrix_generator::recycling_matrix(0.25, 5);
    let test_loop = computations::recycling_loop::RecyclingLoop::new(0.25,0.25, 0.5, 5, true);


    // println!("{}",test);
    println!("{}",test_loop.markov_loop);
    println!("{}",test_loop.limit_loop);
}
