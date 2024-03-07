use std::{io, process};

fn parse_temperature(input_temperature: i32) -> i32 {
    (input_temperature - 32) * 5 / 9
}

fn main() {
    println!("Please write a Farheneit temperature:");
    let mut input_temperature = String::new();
    io::stdin()
        .read_line(&mut input_temperature)
        .expect("Failed to read input");
    let input_temperature: i32 = match input_temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Got unsupported input: {input_temperature}");
            process::exit(1);
        }
    };
    let new_temperature = parse_temperature(input_temperature);
    println!("Celsius temperature is: {new_temperature}");
}
