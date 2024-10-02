fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret_number = 42;
    let mut guess_count = 0;
    
    loop {
        guess_count += 1;
        
        // Simulating user input with predefined guesses
        let mut guess = match guess_count {
            1 => 50,
            2 => 25,
            3 => 37,
            4 => 42,
            _ => break, // Shouldn't reach here, but added for safety
        };
        
        println!("Guess #{}: {}", guess_count, guess);
        
        let result = check_guess(guess, secret_number);
        
        if result == 0 {
            println!("Correct! You've guessed the number!");
            break;
        } else if result == 1 {
            println!("Too high! Try again.");
        } else {
            println!("Too low! Try again.");
        }
    }
    
    println!("Game over! You guessed the number in {} tries.", guess_count);
}