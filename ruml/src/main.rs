use crate::linear_regression::{read_csv, train_loop};

mod linear_regression;

fn main() {
    let data = read_csv();
    let w = 0.0;
    let b = 0.0;
    let learning_rate = 0.0001;
    let iter = 8000;

    let loop_result = train_loop(w, b, learning_rate, data, iter);

    println!("Final Weight : {}", loop_result.0);
    println!("Final Bias : {}", loop_result.1);
}
