use std::time::Instant;

#[no_mangle]
pub fn info() -> String {
    "What is the 10 001st prime number?".to_string()
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

fn get_nth_prime(n: i64) -> i64 {
    let mut counter = 0;
    let mut i = 2;

    while counter < n {
        if is_prime(i) {
            counter += 1;
        }
        i += 1;
    }

    i - 1
}

#[no_mangle]
pub fn solve() -> i64 {
    let now = Instant::now();

    let result = get_nth_prime(10_001);

    println!("Time elapsed: {} seconds", now.elapsed().as_secs_f64());

    result
}