
#[derive(Debug)]
pub struct Student {
    name:String,
    major:String,

}

impl Student{
    pub fn new()-> Student{
        Student{
            name:"Neo".to_string(),
            major: "Cs".to_string(),
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test] fn test_undefined_creation(){
        let s=Student::new
    }
}