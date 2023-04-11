use std::io::{stdin, stdout, Write};

fn check_exit(input: &String) {
    if input == "exit" {
        println!("Exiting program...");
        std::process::exit(0);
    }
}

fn get_user_input() -> String {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}

fn can_convert_to_i32(input: &String) -> Result<(), ()> {
    match input.parse::<i32>() {
        Ok(num) => {
            if num >= 1 && num <= 828 {
                Ok(())
            } else {
                println!("Please enter a number between 1 and 828");
                Err(())
            }
        }
        Err(_) => {
            println!("Input must be a number or 'exit'");
            Err(())
        }
    }
}

pub fn handle_input() -> String {
    loop {
        print!("\nYour selection: ");
        stdout().flush().expect("Failed to flush stdout");

        let input = get_user_input();

        check_exit(&input);

        match can_convert_to_i32(&input) {
            Ok(_) => {
                println!();
                return input
            }
            Err(_) => continue,
        }
    }
}
