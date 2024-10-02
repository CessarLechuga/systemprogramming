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
/*fn sum(total: &mut i32, low: i32, high: i32) {
    for num in low..=high {
        *total += num;
        println!("{}", num);
    }
}

fn main() {
    let mut ans: i32 = 0;
    sum(&mut ans, 0, 10);
    println!("Total sum: {}", ans);
}

*/

/*
fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32) {
    /*
    *total=0;
    let mut low=low;
    while low<=high{
        *total+=low;
        low+=step;

    }
        */
        *total = (low..=high).step_by(step as usize).sum();
}

fn main() {
    let mut result = 0;
    sum_with_step(&mut result, 0, 100, 1);
    println!("Sum 0 to 100, step 1: {}", result);

    result = 0;
    sum_with_step(&mut result, 0, 10, 2);
    println!("Sum 0 to 10, step 2: {}", result);

    result = 0;
    sum_with_step(&mut result, 5, 15, 3);
    println!("Sum 5 to 15, step 3: {}", result);
}
*/
/*

fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> =text.split_whitespace().collect();

    let word = "quick";
    let mut count = 0;

    

    for w in &words {
        if *w == word {
            count += 1;
        }
        println!("{}", w);
    }

    (word.to_string(), count)
    
    //(max_word.to_string, max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}

*/

/*
#[derive(Debug)]
struct Car {
    color: String,
    maker: String,
    year: u16,
}

impl Car {
    fn new(color: String, maker: String, year: u16) -> Car {
        Self {
            color,
            maker,
            year,
        }
    }
    
    fn honk_honk(&self) {
        println!("My car with color {} honk-honk", self.color);
    }

    fn upgrade(&mut self, year: u16) {
        self.year = year;
    }
}

fn main() {
    let mut my_car: Car = Car::new(
        String::from("black"),
        String::from("bmw"),
        2024
    );

    println!("{:?}", my_car);
    my_car.honk_honk();

    my_car.upgrade(2025);
    println!("{:?}", my_car);
}

*/

/*
use std::mem;

struct Car {
    color: String,
    maker: String,
    year: u16,
}



fn main() {
    println!("Size of Car: {} bytes", mem::size_of::<Car>());
    println!("Alignment of Car: {} bytes", mem::align_of::<Car>());
}
    */

    //Write a program Ask user about his or her car create a struc car populate from user input save struct
    //inside of user_infro.txt
    //Read user_info.txt and print the content on the screen 

    /*
    use std::io::{self, Read, Write};
    use std::fs::File;
    
    struct Car {
        make: String,
        model: String,
        year: u32,
    }
    
    fn main() -> io::Result<()> {
        // Get car information from user
        let car = get_car_info()?;
    
        // Save car information to file
        save_car_info(&car)?;
    
        // Read and display car information
        read_and_display_car_info()?;
    
        Ok(())
    }
    
    fn get_car_info() -> io::Result<Car> {
        let mut buffer = String::new();
    
        print!("Enter your car's make: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut buffer)?;
        let make = buffer.trim().to_string();
        buffer.clear();
    
        print!("Enter your car's model: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut buffer)?;
        let model = buffer.trim().to_string();
        buffer.clear();
    
        print!("Enter your car's year: ");
        io::stdout().flush()?;
        io::stdin().read_line(&mut buffer)?;
        let year: u32 = buffer.trim().parse().expect("Please enter a valid year");
    
        Ok(Car { make, model, year })
    }
    
    fn save_car_info(car: &Car) -> io::Result<()> {
        let mut file = File::create("user_info.txt")?;
        writeln!(file, "Make: {}", car.make)?;
        writeln!(file, "Model: {}", car.model)?;
        writeln!(file, "Year: {}", car.year)?;
        Ok(())
    }
    
    fn read_and_display_car_info() -> io::Result<()> {
        let mut file = File::open("user_info.txt")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        println!("\nCar Information:");
        println!("{}", contents);
        Ok(())
    }

    */

    /*
    use std::fs::File;
    use std::io::{self, Read};
    use std::path::Path;

    use std::process::Command;

    fn read_file(path &str)->::Result<string>

    fn executing_os_commands_linux() {
        let output = Command::new("ls")
            .arg("-l")
            .output()
            .expect("Failed to execute command");
    
        println!("Command output: {}", String::from_utf8_lossy(&output.stdout));
    }

    

fn main() {
    executing_os_commands_linux();
}
    */
