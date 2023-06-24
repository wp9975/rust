use std::io;
use rand::Rng;

pub fn generate() {
    let mut rng = rand::thread_rng();

    println!("Choose min number: ");
    let mut min = String::new();
    io::stdin().read_line(&mut min).unwrap();
    let min: i32 = min.trim().parse().unwrap();

    println!("Choose max number: ");
    let mut max = String::new();
    io::stdin().read_line(&mut max).unwrap();
    let max: i32 = max.trim().parse().unwrap();

    let x = rng.gen_range(min..max);

    println!("{}", x);
}
