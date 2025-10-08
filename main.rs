use std::io::{self, Write}; // We need to import Write for the flush() function

fn main() {
    // Define the maximum safe input for u64 factorial (20! is the largest that fits)
    const MAX_FACTORIAL_INPUT: u64 = 20;

    loop {
        // Prompt the user for input and flush the output to ensure the text appears immediately
        print!("Enter a positive integer (up to {}) to calculate its factorial (or 'exit' to quit): ", MAX_FACTORIAL_INPUT);
        io::stdout().flush().unwrap();
        
        let mut input = String::new(); // Create an empty string for user input
        
        // Read the input from the user
        io::stdin().read_line(&mut input)
            .expect("Failed to read line"); 
            
        let input_trimmed = input.trim();
        
        // 1. Handle the 'exit' command
        if input_trimmed.eq_ignore_ascii_case("exit") {
            println!("Exiting the program. Goodbye! 👋");
            break; // Exits the loop and ends the program
        }

        // 2. Convert the input string to a u64 number
        let number: u64 = match input_trimmed.parse() {
            Ok(num) => num,
            Err(_) => {
                // If parsing fails (e.g., non-numeric input), print error and continue the loop
                println!("Invalid input. Please enter a positive integer or 'exit'.");
                continue; // Jumps to the start of the loop
            }
        };

        // 3. Check for potential overflow *before* calculation (since 21! overflows u64)
        if number > MAX_FACTORIAL_INPUT {
            println!("The factorial of {} will cause an overflow. Please enter a number up to {}.", number, MAX_FACTORIAL_INPUT);
            continue; // Jumps to the start of the loop
        }

        // The factorial of 0 is defined as 1
        if number == 0 {
            println!("The factorial of 0 is 1.");
            continue; // Jumps to the start of the loop
        }
        
        // --- Factorial Calculation ---
        let mut factorial: u64 = 1;
        
        // Iterative calculation: factorial = 1 * 2 * 3 * ... * number
        for i in 1..=number {
            // Using checked_mul() for safe multiplication
            match factorial.checked_mul(i) {
                Some(res) => factorial = res,
                None => {
                    // This block should ideally not be reached due to the MAX_FACTORIAL_INPUT check
                    println!("Error: Internal overflow detected for number {}.", number);
                    break;
                }
            }
        }
        
        println!("The factorial of {} is: {}", number, factorial);
    }
}