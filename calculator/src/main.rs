use std::io::{self, Write}; // For handling user input/output

// Define the Operation enum with variants Add, Subtract, Multiply, and Divide
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Function to calculate the result based on the operation
fn calculate(operation: Operation) -> f64 {
    match operation {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => {
            if b != 0.0 {
                a / b
            } else {
                println!("Error: Cannot divide by zero.");
                std::f64::NAN // Return NaN in case of division by zero
            }
        }
    }
}

fn main() {
    // Prompt the user to input the first number
    print!("Enter the first number: ");
    io::stdout().flush().unwrap(); // Ensure prompt is printed before input
    let mut first_input = String::new();
    io::stdin().read_line(&mut first_input).unwrap();
    let first_number: f64 = first_input.trim().parse().unwrap_or_else(|_| {
        println!("Invalid input for the first number.");
        std::process::exit(1);
    });

    // Prompt the user to input the operation
    print!("Enter the operation (+, -, *, /): ");
    io::stdout().flush().unwrap();
    let mut operation_input = String::new();
    io::stdin().read_line(&mut operation_input).unwrap();
    let operation = operation_input.trim();

    // Prompt the user to input the second number
    print!("Enter the second number: ");
    io::stdout().flush().unwrap();
    let mut second_input = String::new();
    io::stdin().read_line(&mut second_input).unwrap();
    let second_number: f64 = second_input.trim().parse().unwrap_or_else(|_| {
        println!("Invalid input for the second number.");
        std::process::exit(1);
    });

    // Create the Operation enum instance based on the user's input
    let operation_enum = match operation {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid operation.");
            std::process::exit(1);
        }
    };

    // Call the calculate function with the created Operation enum instance
    let result = calculate(operation_enum);

    // Print the result to the console
    println!("Result: {}", result);
}
