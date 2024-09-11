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
       let guess = 50 - guess_count; // Simulating user input
       
       match check_guess(guess, secret_number) {
           0 => {
               println!("Correct! The number was {}", secret_number);
               break;
           }
           1 => println!("Too high! Try again."),
           -1 => println!("Too low! Try again."),
           _ => unreachable!(),
       }
   }
   
   println!("You guessed the number in {} tries", guess_count);
}