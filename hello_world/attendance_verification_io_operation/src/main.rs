 //Write a program Ask user about his or her car create a struc car populate from user input save struct
    //inside of user_infro.txt
    //Read user_info.txt and print the content on the screen 

    
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