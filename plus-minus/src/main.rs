use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plusMinus(arr: &[i32]) {
    let mut p: f64 = 0.0;
    let mut n: f64 = 0.0;
    let mut z: f64 = 0.0;
    for i in arr {
        if i > &0 {
            p+= 1 as f64
        }
        else if i < &0 {
            n+= 1 as f64
        }
        else {
            z+= 1 as f64
        }
    }
    println!("{:.6}\n{:.6}\n{:.6}", p as f64 / arr.len() as f64, n as f64 / arr.len() as f64, z as f64 / arr.len() as f64);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plusMinus(&arr);
}
