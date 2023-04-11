use std::time::Instant;

#[no_mangle]
pub fn info() -> String {
    "Find the thirteen adjacent digits in the 1000-digit number that have the greatest product. \n\
    What is the value of this product?".to_string()
}

fn find_largest_product(number: &str, digits: usize) -> i64 {
    let mut largest_product = 0;
    let mut product = 1;
    let mut i = 0;
    let mut j = digits -1;
    let base_10 = 10;
    while j < number.len() {
        for digit in number[i..=j].chars() {
            product *= digit.to_digit(base_10).unwrap() as i64;
        }
        if product > largest_product {
            largest_product = product;
        }
        product = 1;
        i += 1;
        j += 1;
    }
    largest_product
}

#[no_mangle]
pub fn solve() -> i64 {
    // which number to begin?
    let number = "7316717653133062491922511967442657474235534919493496983520312774506326239578318016984801869478851843858615607891129494954595017379583319528532088055111254069874715852386305071569329096329522744304355766896648950445244523161731856403098711121722383113622298934233803081353362766142828064444866452387493035890729629049156044077239071381051585930796086670172427121883998797908792274921901699720888093776657273330010533678812202354218097512545405947522435258490771167055601360483958644670632441572215539753697817977846174064955149290862569321978468622482839722413756570560574902614079729686524145351004748216637048440319989000889524345065854122758866688116427171479924442928230863465674813919123162824586178664583591245665294765456828489128831426076900422421902267105562632111110937054421750694165896040807198403850962455444362981230987879927244284909188845801561660979191338754992005240636899125607176060588611646710940507754100225698315520005593572972571636269561882670428252483600823257530420752963450";
    let now = Instant::now();

    let result = find_largest_product(number, 13);

    // return solution as i128
    println!("Time elapsed: {} seconds", now.elapsed().as_secs_f64());
    result
}