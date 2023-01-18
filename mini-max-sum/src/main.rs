use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn miniMaxSum(arr: &[i32]) {
    let mut nw = arr.to_vec();
    let mut ne = arr.to_vec();
    let mut min:i64 = 0;
    let mut max:i64 = 0;
    //Min
    nw.sort();
    nw.pop();
    min = nw.iter().map(|x| *x as i64).sum();
    //Max
    ne.sort_by(|a,b|b.cmp(a));
    ne.pop();
    max = ne.iter().map(|x| *x as i64).sum();
    println!("{} {}", min, max);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}
