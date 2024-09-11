/*const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + FREEZING_POINT_F
}

fn main() {
    let mut temperature_f = 32.0;
    
    println!("{}°F is equal to {:.2}°C", temperature_f, fahrenheit_to_celsius(temperature_f));
    
    for _ in 0..5 {
        temperature_f += 1.0;
        println!("{}°F is equal to {:.2}°C", temperature_f, fahrenheit_to_celsius(temperature_f));
    }
}
    */

    /*
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }
    
    fn main() {
        let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        
        for &num in &numbers {
            if num % 3 == 0 && num % 5 == 0 {
                println!("FizzBuzz");
            } else if num % 3 == 0 {
                println!("Fizz");
            } else if num % 5 == 0 {
                println!("Buzz");
            } else if is_even(num) {
                println!("{} is even", num);
            } else {
                println!("{} is odd", num);
            }
        }
        
        let mut sum = 0;
        let mut i = 0;
        while i < numbers.len() {
            sum += numbers[i];
            i += 1;
        }
        println!("Sum of all numbers: {}", sum);
        
        let mut largest = numbers[0];
        let mut j = 1;
        loop {
            if j >= numbers.len() {
                break;
            }
            if numbers[j] > largest {
                largest = numbers[j];
            }
            j += 1;
        }
        println!("Largest number: {}", largest);
    }
*/    







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