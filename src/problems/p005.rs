use std::time::Instant;

#[no_mangle]
pub fn info() -> String {
    "Smallest positive number that is evenly divisible by all of the numbers from 1 to 20.".to_string()
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
pub fn solve() -> i128 {
    let now = Instant::now();

    let mut num = 20;

    loop {
        let mut is_divisible = true;
        for i in 1..=20 {
            if num % i != 0 {
                is_divisible = false;
                break;
            }
        }
        if is_divisible {
            break;
        }
        num += 20;
    }

    println!("Time elapsed: {} seconds", now.elapsed().as_secs_f64());
    num as i128
}