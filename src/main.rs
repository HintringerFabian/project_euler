mod user;
mod dynamic_loading;

use crate::dynamic_loading::run_problem;

pub fn print_info() {
    println!("In the problem folder you can see the numbers of the problems in the file names.");
    println!("The problem numbers are the same as on projecteuler.net");
    println!("Please select a problem to run by typing the number of the problem.");
    println!("For example, to run problem 1, type 1 and press enter.");
    println!("To exit, type 'exit' and press enter.");
}

fn main() {
    print_info();
    let input = user::handle_input();
    run_problem(input);
}
