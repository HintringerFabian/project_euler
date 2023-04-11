use std::time::Instant;

#[no_mangle]
pub fn info() -> String {
    "".to_string()
}

#[no_mangle]
pub fn solve() -> i64 {
    // which number to begin?
    let x;
    let now = Instant::now();

    // calculate solution

    // return solution as i128
    println!("Time elapsed: {} seconds", now.elapsed().as_secs_f64());
    x
}