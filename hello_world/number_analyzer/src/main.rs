fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers = [12, 7, 15, 30, 22, 9, 45, 60, 3, 18];

    println!("Analyzing numbers:");
    for &num in &numbers {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println!("{}: Buzz", num);
        } else if is_even(num) {
            println!("{}: Even", num);
        } else {
            println!("{}: Odd", num);
        }
    }

    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    }
    println!("\nSum of all numbers: {}", sum);

    let mut largest = numbers[0];
    for &num in &numbers[1..] {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number: {}", largest);
}
