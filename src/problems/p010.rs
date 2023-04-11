use std::time::Instant;

#[no_mangle]
pub fn info() -> String {
    "Find the sum of all the primes below two million.".to_string()
}

fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    } else if n <= 3 {
        return true;
    } else if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

#[no_mangle]
pub fn solve() -> i64 {
    let now = Instant::now();

    let result = (1..2000000).filter(|&x| is_prime(x)).sum::<i64>();

    // return solution as i128
    println!("Time elapsed: {} seconds", now.elapsed().as_secs_f64());
    result
}