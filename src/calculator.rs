use std::io;

pub fn calculator() {
    println!("Podaj pierwszą liczbę:");
    let mut num1 = String::new();
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let num1: i32 = num1.trim().parse().expect("Please type a number!");

    println!("Podaj drugą liczbę:");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num2: i32 = num2.trim().parse().expect("Please type a number!");

    println!("Dodawanie: {}", add(num1, num2));
    println!("Odejmowanie: {}", subtract(num1, num2));
    println!("Mnożenie: {}", multiply(num1, num2));
    println!("Potęgowanie: {}", power(num1, num2));
    println!("Modulo: {}", modulo(num1, num2));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

fn power(a: i32, b: i32) -> i32 {
    a.pow(b as u32)
}

fn modulo(a: i32, b: i32) -> i32 {
    a % b
}
