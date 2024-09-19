use std::io;
use std::str::FromStr;

type CustomResult = Result<u32, String>;

fn conver_to_num(number_str : &str) -> Result<u32, <u32 as FromStr>::Err> {
    let number = number_str.trim().parse().expect("Invalid number");
    Ok(number)
}

fn multiply_by_two(num : u32) -> CustomResult {
    let x = num.checked_mul(2);
    
    if x.is_some(){
        let x = x.unwrap();
        Ok(x)
    } else {
        Err("Overflow".to_string())
    }
}

fn main() {
    println!("Welcome to an optional calculator!");
    println!("Enter a number: ");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("error: unable to read input");
    let num = conver_to_num(&input).unwrap();
    let result = multiply_by_two(num);
    println!("Multiply result {:?}", result);
}