use std::io::{self, BufRead};

fn main() {
    let reader = io::stdin();
    let numbers: Vec<i32> = 
        reader.lock()                           
              .lines().next().unwrap().unwrap() 
              .split(' ').map(|s| s.trim())    
              .filter(|s| !s.is_empty())       
              .map(|s| s.parse().unwrap())      
              .collect();

    println!("{}", gcd(numbers[0], numbers[1]));
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}

