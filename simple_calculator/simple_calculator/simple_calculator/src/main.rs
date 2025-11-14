use std::io;         // Bring the `io` (input/output) module into scope from the standard library.
use std::io::Write;  // Bring the `Write` trait into scope so we can call `flush()` on stdout.

// The main function is the entry point of every Rust program.
// When you run `cargo run`, Rust starts executing from here.
fn main() {
    // Print a welcome message. `println!` is a macro that prints a line and adds a newline at the end.
    println!("Simple calculator");
    println!("================\n");

    // `loop` creates an infinite loop. It will run forever until we explicitly `break` from it.
    // Here it allows the user to perform multiple calculations without restarting the program.
    loop {
        // ===== 1. Read the first number from the user =====

        // `print!` (without ln) prints text but does NOT add a newline.
        // We use this so the user's input appears on the same line as the prompt.
        print!("Enter first number (or 'q' to quit): ");

        // `io::stdout()` gives us a handle to the standard output.
        // `flush()` forces any buffered output to be written immediately to the screen.
        // Without flushing, sometimes the prompt might not appear before `read_line` waits for input.
        io::stdout().flush().expect("Failed to flush stdout");

        // Call our helper function to read a line of input from the user.
        let input = read_input();
        // `.trim()` removes whitespace from both ends of the string (like `\n` from pressing Enter).
        let input = input.trim();

        // Allow the user to quit the program by typing 'q' or 'Q'.
        if input.eq_ignore_ascii_case("q") {
            println!("Goodbye!");
            break; // Exit the loop (and thus end the program).
        }

        // Try to convert the input string into a floating-point number (`f32`).
        // `parse::<f32>()` returns a `Result<f32, _>`, because parsing can fail (e.g. "abc").
        let first_num = match input.parse::<f32>() {
            // `Ok(num)` means parsing succeeded; `num` is the parsed `f32` value.
            Ok(num) => num,
            // `Err(_)` means parsing failed. We don't care about the specific error, so we use `_`.
            Err(_) => {
                println!("Invalid input. Please enter a number.\n");
                // `continue` jumps back to the beginning of the loop, skipping the rest of this iteration.
                continue;
            }
        };

        // ===== 2. Read the second number from the user =====

        print!("Enter second number: ");
        io::stdout().flush().expect("Failed to flush stdout");

        // Directly read, trim, and parse the second number.
        let second_num = match read_input().trim().parse::<f32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.\n");
                continue;
            }
        };

        // ===== 3. Read the operation (+, -, *, /) =====

        print!("Enter operation (+, -, *, /): ");
        io::stdout().flush().expect("Failed to flush stdout");

        let operation = read_input();
        // `trim()` removes whitespace at the ends; e.g., the newline from hitting Enter.
        let operation = operation.trim();

        // ===== 4. Perform the calculation =====

        // `calculate` returns a `Result<f32, String>`.
        // This means it might succeed (`Ok(result)`) or fail (`Err(error_message)`).
        let result = calculate(first_num, second_num, operation);

        // Handle both success and error cases using `match` on the `Result`.
        match result {
            // On success, `res` is the computed `f32` value.
            Ok(res) => println!("Result: {}\n", res),
            // On error, `err_msg` is the descriptive error string we returned from `calculate`.
            Err(err_msg) => println!("Error: {}\n", err_msg),
        }
    }
}

// This function reads one line of text from standard input and returns it as a `String`.
fn read_input() -> String {
    // Create a mutable, empty `String` to store the input.
    // `mut` means we can modify the value after it is created.
    let mut input = String::new();

    // `io::stdin()` gives us a handle to standard input (the keyboard).
    // `read_line(&mut input)` reads user input until they press Enter,
    // and appends it to the `input` string.
    // `&mut input` is a mutable reference, meaning the function is allowed to modify `input`.
    io::stdin()
        .read_line(&mut input)
        // `expect` will crash the program with the given message if an error occurs during input.
        .expect("Failed to read line");

    // Return the string containing the user's input.
    input
}

// This function performs the actual calculation.
// It takes:
// - `first_num`: the first number (f32)
// - `second_num`: the second number (f32)
// - `operation`: a string slice (`&str`) that should be one of "+", "-", "*", "/"
// It returns a `Result<f32, String>`:
// - `Ok(result)` if the operation is valid and succeeds
// - `Err(error_message)` if something goes wrong (e.g., division by zero or invalid operator)
fn calculate(first_num: f32, second_num: f32, operation: &str) -> Result<f32, String> {
    // Before doing the division, we check for division by zero.
    // In math, dividing by zero is undefined, and in programming, it's usually an error.
    if operation == "/" && second_num == 0.0 {
        // `return` immediately exits the function, returning this value.
        // Here we return an error variant of `Result` with a descriptive message.
        return Err("Division by zero is not allowed".to_string());
    }

    // Use a `match` expression to choose what to do based on the `operation` string.
    // `match` is like a more powerful `switch` from other languages.
    let result = match operation {
        // If the user typed "+", add the two numbers.
        "+" => first_num + second_num,
        // If "-", subtract the second from the first.
        "-" => first_num - second_num,
        // If "*", multiply them.
        "*" => first_num * second_num,
        // If "/", we already know second_num is not zero (checked above), so divide.
        "/" => first_num / second_num,
        // `_` is a "catch-all" pattern that matches anything not caught by previous patterns.
        // If the user typed something else, we return early with an error.
        _ => {
            return Err(format!("Invalid operation: '{}'", operation));
        }
    };

    // If we reach here, everything went fine, and `result` holds the answer.
    // `Ok(result)` wraps the value in the success variant of `Result`.
    Ok(result)
}