#[no_mangle]
pub fn info() -> String {
    "Find the sum of all the multiples of 3 or 5 below 1000.".to_string()
}

#[no_mangle]
pub fn solve() -> i64 {
    let n = 1000;

    (1..n).filter(|x| x % 3 == 0 || x % 5 == 0).sum::<i64>()
}