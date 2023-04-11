use std::time::Instant;

#[no_mangle]
pub fn info() -> String {
    "Find the largest palindrome made from the product of two 3-digit numbers.".to_string()
}

fn is_palindrome(num: i64) -> bool {
    let num_str = num.to_string();
    let num_str_rev = num_str.chars().rev().collect::<String>();
    num_str == num_str_rev
}

#[no_mangle]
pub fn solve() -> i64 {
    let mut largest_palindrome = 0;

    let now = Instant::now();

    for i in 100..1000 {
        for j in 100..1000 {
            let product = i * j;
            if is_palindrome(product) && product > largest_palindrome {
                largest_palindrome = product;
            }
        }
    }

    println!("Time elapsed: {} seconds", now.elapsed().as_secs_f64());

    largest_palindrome
}