use std::io;
use rand::Rng; // import the RNG trait

fn main() {
    // build an interactive guessing game in Rust
    // random number generation, user input, control flow
    let secret_number = num_gen();
    let mut counter: i32 = 0;
    let mut won = false;
    const MAX_GUESSES: i32 = 10;
    while counter < MAX_GUESSES {
        println!("Guess the number!");
        println!("You have {} guesses left!", MAX_GUESSES - counter);
        let guess: i32 = read_input();
        if guess == secret_number {
            won = true;
            break;
        } else if guess > secret_number {
            println!("Too high!\n");
        } else {
            println!("Too low!\n");
        }
        counter += 1;
    }
    if won == false {
        println!("Game over!");
        println!("The secret number was: {}", secret_number);
    } else {
        println!("You win!");
    }
}

fn num_gen() -> i32 {
    rand::thread_rng().gen_range(1..=101)
}

fn read_input() -> i32 {
    loop {
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        match guess.trim().parse::<i32>() {
            Ok(num) => {
                return num
            },
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        }
    }
}

