use std::io::{self, Read};

fn main() {
    println!("Simple calculator");

    loop {
        // Reading the user input

        println!("Please input you operation (add, subtract, multiply, divide) or 'exit' to quit:");

        let mut operation = String::new();
        io::stdin()
            .read_line(&mut operation)
            .expect("Failed to read line");

        // Trimming and checking Input
        let operation = operation.trim();
        if operation == "exit" {
            break;
        }

        // Reading Numbers

        println!("Enter the first number:");

        let mut num1 = String::new();
        io::stdin()
            .read_line(&mut num1)
            .expect("Failed to read line");
        let num1: f64 = match num1.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                continue;
            }
        };

        println!("Enter the second number");

        let mut num2 = String::new();
        io::stdin()
            .read_line(&mut num2)
            .expect("Failed to read line");
        let num2: f64 = match num2.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number");
                continue;
            }
        };

        // Performing the calculation
        let result = match operation {
            "add" => num1 + num2,
            "subtract" => num1 - num2,
            "multiply" => num1 * num2,
            "divide" => {
                if num2 == 0.0 {
                    println!("Cannot divide by zero");
                    continue;
                }
                num1 / num2
            }
            _ => {
                println!("Invalid operation");
                continue;
            }
        };

        println!(
            "The result of {} {} {} is: {}",
            num1, operation, num2, result
        )
    }
}
