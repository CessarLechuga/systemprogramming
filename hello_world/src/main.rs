

/*enum Pets {
    Dog(String, u8),
    Hamster(String), 
}

impl Pets {
    fn introduce_yourself(&self) {
        match self {
            Pets::Dog(name, age) => {
                println!("Hey my name is {}. I am {} years old", name, age);
            },
            Pets::Hamster(name) => println!("Hey my name is {}", name),
        }
    }
}

fn main() {
    let p1 = Pets::Dog(String::from("Black"), 3);

    p1.introduce_yourself();
    p1.introduce_yourself();
}

*/

/*

use std::fs;
use std::io::{self, Write};
use std::path::Path;

enum FileOperation {
    Create(String),
    Rename(String, String),
    Write (String),
}

fn get_user_input{
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input=String::new()
    
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::Create(filename) => {
            // TODO: Implement file creation logic
            if FileOperation::validate_file(&filename){
                println!("File alreay exist");
                return;
            }else{
            fs::File::create(&filename).unwrap();

            println!("File '{}' created successfully.", filename);
            }
        }
        FileOperation::Rename(old_name, new_name) => {
            // TODO: Implement file renaming logic
            if !Path::new(&old_name).exists() {
            println!("File '{}' does not exist.", old_name);
            
            }else if Path::new(&new_name).exists() {
                println!("File '{}' already exists.", new_name);
            }
            else{
                println!("File renamed from '{}' to '{}' successfully.", old_name, new_name);
                
            }

        }
    }
}

fn main() {
    get_user_input()->String{
    println!("Choose an operation:");
    println!("1. Create a new file");
    println!("2. Rename an existing file");
    }

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    let mut choice: String= FileOperation::get_user_input

    match choice.trim() {
        "1" => {
            // TODO: Prompt for new filename and call perform_operation
            let new_file:String=FileOperation::get_user_input();
            perform_operation(FileOperation::Create(new_file))
        }
        "2" => {
            // TODO: Prompt for old and new filenames and call perform_operation
            println!("Type the type of ")
        }
        _ => println!("Invalid choice"),
    }
}

*/

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    println!("2 + 2 = {}", add(2, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}