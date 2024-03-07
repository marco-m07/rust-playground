// Goal: Generate the nth Fibonacci number.

use std::io;

fn get_fibonacci_number(end: usize) -> u128 {
    let mut index: usize = 0;
    let mut fib_number: u128 = 0;
    let mut fib_sequence: Vec<u128> = Vec::new();
    while index <= end {
        if index == 1 {
            fib_number = 1;
        } else if index > 1 {
            fib_number = fib_sequence[index - 1] + fib_sequence[index - 2];
        }
        fib_sequence.push(fib_number);
        index = index + 1;
    }
    fib_number
}

fn main() {
    println!("Which Fibonacci number would you like to generate?");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read input");
    let n: usize = n.trim().parse().expect("Please type a number!");
    let fib_number = get_fibonacci_number(n);
    println!("The {}th Fibonacci number is {}", n, fib_number);
}
