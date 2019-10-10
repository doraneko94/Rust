extern crate rand;
use rand::random;
use std::error::Error;
use std::process;
use std::fs::File;

pub fn csv_to_vec() -> Vec<Vec<f32>> {
    let file_path = "../load.csv";
    let file = File::open(file_path).expect("file not found");
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(file);
    let mut x = Vec::new();
    for result in rdr.records() {
        let record = result.expect("file not found");
        let v: Vec<f32> = record.iter().map(|e| e.parse().ok().unwrap()).collect();
        x.push(v);
    }
    x
}

fn write_run(values: &Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path("../save.csv")?;
    for row in values {
        wtr.write_record(row)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn vec_to_csv<T: ToString>(values :&Vec<Vec<T>>) {
    let v_str: Vec<Vec<String>> = values.iter().map(|v| v.iter().map(|e| e.to_string()).collect()).collect();
    if let Err(err) = write_run(&v_str) {
        println!("{}", err);
        process::exit(1);
    }
}

pub fn init_matrix_2d(n_row: i32, n_col: i32) -> Vec<Vec<f32>> {
    let mut x = Vec::with_capacity(n_row as usize);
    for i in 0..n_row {
        let mut col = Vec::with_capacity(n_col as usize);
        for j in 0..n_col {
            col.push(random::<f32>());
        }
        x.push(col);
    }
    x
}

pub fn init_matrix_3d(n1: i32, n2: i32, n3: i32) -> Vec<Vec<Vec<f32>>> {
    let mut x = Vec::with_capacity(n1 as usize);
    for i in 0..n1 {
        let mut d2 = Vec::with_capacity(n2 as usize);
        for j in 0..n2 {
            let mut d3 = Vec::with_capacity(n3 as usize);
            for k in 0..n3 {
                d3.push(random::<f32>());
            }
            d2.push(d3);
        }
        x.push(d2);
    }
    x
}