use std::time::Instant;

#[no_mangle]
pub fn info() -> String {
    "Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. \n\
    What is the value of this product?".to_string()
}

fn pythagorean_triplet() -> i64 {
    let mut a: i32 = 1;
    let mut b: i32 = 2;
    let mut c: i32 = 3;

    while a < 1000 {
        while b < 1000 {
            while c < 1000 {
                if a + b + c == 1000 && a.pow(2) + b.pow(2) == c.pow(2) {
                    return (a * b * c) as i64;
                }
                c += 1;
            }
            b += 1;
            c = b + 1;
        }
        a += 1;
        b = a + 1;
        c = a + 2;
    }

    0
}

#[no_mangle]
pub fn solve() -> i64 {
    let now = Instant::now();

    let result = pythagorean_triplet();

    println!("Time elapsed: {} seconds", now.elapsed().as_secs_f64());

    result
}