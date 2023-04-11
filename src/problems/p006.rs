use std::time::Instant;

#[no_mangle]
pub fn info() -> String {
    "The difference between the sum of the squares of the first one hundred natural \n\
    numbers and the square of the sum.".to_string()
}

fn sqaure_of_sum(n: i64) -> i64 {
    let sum = (1..=n).sum::<i64>();
    sum * sum
}

fn sum_of_squares(n: i64) -> i64 {
    (1..=n).map(|x| x * x).sum::<i64>()
}

#[no_mangle]
pub fn solve() -> i64 {
    let now = Instant::now();
    let result = sqaure_of_sum(100) - sum_of_squares(100);
    println!("Time elapsed: {} seconds", now.elapsed().as_secs_f64());

    result
}