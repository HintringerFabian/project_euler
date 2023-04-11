#[no_mangle]
pub fn info() -> String {
    "Find the largest prime factor of the number 600851475143".to_string()
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

fn get_divisors(num: i64) -> i64 {
    if num <= 1 {
        panic!("Number must be greater than 1")
    }

    if num % 2 == 0 {
        return 2;
    }

    let sqrt_num = (num as f64).sqrt() as i64;

    for i in (3..sqrt_num).step_by(2) {
        if num % i == 0 && is_prime(i) {
            return i;
        }
    }

    num
}

#[no_mangle]
pub fn solve() -> i64 {
    let mut num: i64 = 600851475143;

    loop {
        let divisors = get_divisors(num);
        if divisors < num {
            num /= divisors;
        } else {
            return num;
        }
    }
}