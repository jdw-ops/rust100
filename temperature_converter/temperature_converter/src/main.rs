use std::io;
use std::io::Write;

fn main() {
    // Display welcome messages to the user
    println!("Hello, user");
    println!("This is the temperature converter.");

    // Prompt user to choose their unit of measurement
    print!("Choose your unit (fahrenheit or celsius): ");
    // Flush stdout to ensure the prompt appears before waiting for input
    io::stdout().flush().expect("Failed to flush stdout");
    // Create a mutable String to store the user's unit choice
    let mut unit = String::new();
    // Read a line from standard input into the unit variable
    io::stdin().read_line(&mut unit).expect("Failed to read line");

    let unit = unit.trim().to_lowercase();

    // Prompt user to enter the temperature value
    print!("Enter temperature: ");
    // Flush stdout to display the prompt immediately
    io::stdout().flush().expect("Failed to flush stdout");
    // Create a mutable String to store the temperature input
    let mut temp = String::new();
    // Read the temperature value from standard input
    io::stdin().read_line(&mut temp).expect("Failed to read line");

    // Parse the temperature string into a floating-point number (f64).
    // Use a `match` to handle potential parsing errors gracefully.
    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid temperature. Please enter a number.");
            return; // Exit the program if input is not a valid number
        }
    };

    // Check if user wants to convert FROM fahrenheit TO celsius
    if unit == "fahrenheit" {
        // Parse string to i32 and apply fahrenheit to celsius formula: (F - 32) * 5/9
        let result = (temp - 32.0) * 5.0 / 9.0;
        // Display the converted temperature
        println!("The temperature in celsius is {:.2} degrees.", result); // Check if user wants to convert FROM celsius TO fahrenheit
    } else if unit == "celsius" {
        // Parse string to i32 and apply celsius to fahrenheit formula: C * 9/5 + 32
        let result = temp * 9.0 / 5.0 + 32.0;
        // Display the converted temperature
        println!("The temperature in fahrenheit is {:.2} degrees.", result);
    // Handle invalid input
    } else {
        println!("Invalid unit of measurement");
    }
    println!("Thank you for using the temperature converter.");
}
