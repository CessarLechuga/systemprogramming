fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(x:i32,y:i32)->i32{
let mut total:i32=0;

for _ in 0..y {
    total = add(total, x);
}
total
}

fn exponentiation(x:i32,y:i32)->f32{
    if power<0{
        return 1 as f32/exponentiation(base,-power);
    }
    let mut total:i32=1;
    
    for _ in 0..y {
        total = multiply(total, x);
    }
    total
    }

fn main() {
    println!("2 + 2 = {}", add(2, 2));
    println!("2 * 2 = {}", multiply(2, 2));
    println!("2**2 = {}", exponentiation(2, 2));
    println!("5**-2 = {}", exponentiation(5, -2));
}

#[cfg(test)]
mod tests {
    use super::*;

    mod test_add_functionality{
        use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }

}

    #[test]
    fn test_multiply(){
        assert_eq!(multiply(5,2),10);

    }

    #[test]
    fn test_exponential(){
        assert_eq!(exponentiation(2,3),8);
    }


}

