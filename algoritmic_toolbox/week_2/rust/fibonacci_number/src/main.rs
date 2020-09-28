fn main() {
    let a = get_input().trim().parse::<i32>().unwrap();
    println!("{}", fib(a));
}

fn fib(n: i32) -> i32 {
    let mut a = 0;
    let mut b = 1;
    let mut c;

    if n == 0 {
        return a;
    }

    for _i in 2..=n {
        c = a + b;
        a = b;
        b = c;
    }

    return b;
}

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}
