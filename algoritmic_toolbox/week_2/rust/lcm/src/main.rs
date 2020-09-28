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

    println!("{}", lcm(numbers[0], numbers[1]));
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}

fn lcm(a: i32, b: i32) -> u64 {
    (a as u64 * b as u64) / gcd(a, b) as u64
}
