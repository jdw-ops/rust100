use std::io;
use rand::Rng; // import the RNG trait


fn num_gen() -> i32 {
    let rng = rand::thread_rng().gen_range(1..=101);
    rng
}

fn read_input() -> i32 {
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return guess;
    }
}

fn main() {
    // build an interactive guessing game in Rust
    // random number generation, user input, control flow
    let secret_number = num_gen();
    let mut counter: i32 = 0;
    while counter < 10 {
        println!("Guess the number!");
        println!("You have {} guesses left!", 10 - counter);
        let guess = read_input();
        if guess == secret_number {
            println!("You win!");
            break;
        }
        counter += 1;
    }
    println!("Game over!");
    println!("The secret number was: {}", secret_number);
}
