use std::io;

fn fibonacci(n: u128) -> u128 {
    let mut a: u128 = 0;
    let mut b: u128 = 1;
    for _ in 0..n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    a
}

fn main() {
    println!("Find the nth Fibonacci number. Enter n:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed read input.");
    let n: u128 = input.trim().parse().expect("Input valid number.");

    let fibonacci_number = fibonacci(n);
    println!("{}th Fibonacci number: {}", n, fibonacci_number);
}
