extern crate seqnmf;
use std::convert::TryInto;
use seqnmf::{vec_to_csv, csv_to_vec, init_matrix_2d, init_matrix_3d};

fn main() {
    // let max_iter = 30;
    // let test_size: f32 = 0.1;
    // let x_filename = "load.csv";
    let mut x = csv_to_vec();
    let n: i32 = x.len().try_into().unwrap();
    let t: i32 = x[0].len().try_into().unwrap();
    // let t_test: i32 = ((t as f32) * test_size).round() as i32;
    // let mut test_flag = false;
    // if t_test > 0 {
        // t -= t_test;
        // test_flag = true;
    // }
    let k: i32 = 2;
    let l: i32 = 5;
    let lambda_min = 0.00001;
    let lambda_max = 1.0;
    let iter_per_lambda = 5;
    let mut lambda_vec = Vec::new();
    let mut lamb = lambda_min;
    while lamb <= lambda_max {
        for i in 0..iter_per_lambda {
            lambda_vec.push(lamb);
        }
        lamb *= 10.0;
    }
    let lambda_w = 0.1;
    // println!("{}", t_test);
    x[0][1] = 0.00001;
    let mut w = init_matrix_3d(n, k, l);
    let mut h = init_matrix_2d(k, t);

    vec_to_csv(&h);
}