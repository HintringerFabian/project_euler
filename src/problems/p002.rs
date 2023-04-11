use std::collections::HashMap;

#[no_mangle]
pub fn info() -> String {
    "By considering the terms in the Fibonacci sequence whose values do not exceed four million,\n\
    find the sum of the even-valued terms.".to_string()
}

fn fibo(n: u32, memory: &mut HashMap<i64, i64>) -> i64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibo(n - 1, memory) + fibo(n - 2, memory)
    }
}

#[no_mangle]
pub fn solve() -> i64 {
    let mut map: HashMap<i64, i64> = HashMap::new();

    let mut n: u32 = 0;
    let mut sum: i64 = 0;
    let mut fibo_num: i64 = fibo(0, &mut map);

    while fibo_num < 4_000_000 {

        if fibo_num % 2 == 0 {
            sum += fibo_num;
        }
        n += 1;

        fibo_num = fibo(n, &mut map);
    }

    sum
}