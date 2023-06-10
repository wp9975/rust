// Module: basics
// File: src/basics.rs




pub mod basics{

    fn five() -> i32 {
        5
    }
    
    fn plus_one(x: i32) -> i32 {
        x + 1
    }

pub fn basics1(){
    println!("Hello, world!");
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);
    let x = 2.0;
    let y: f32 = 3.0;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    let t = true;
    let f: bool = false;
    println!("The value of t is: {}", t);
    println!("The value of f is: {}", f);
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("The value of c is: {}", c);
    println!("The value of z is: {}", z);
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);
    let a = [3; 5];
    let first = a[0];
    let second = a[1];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);
    let a = [1, 2, 3, 4, 5];
    let index = 4; 
    let element = a[index];
    println!("The value of element is: {}", element);
    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    let x = five();
    println!("The value of x is: {}", x);
    let x = plus_one(5);
    println!("The value of x is: {}", x);
    let number = 3;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
    
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
    let s1 = String::from("hello");
    let s2 = s1.clone(); // używamy metody clone
    println!("{}, world!", s1); 
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);
}

pub fn basics2(){
    let name = "Alice"; // immutable variable
    let mut age = 20; // mutable variable
    println!("Hello, {}!", name);
    println!("You're {} years old.", age);

    age = age + 1;
    println!("Next year, you'll be {}.", age);

    greet("Bob");

    let result = add(5, 7);
    println!("5 + 7 = {}", result);

    let numbers = vec![1, 2, 3, 4, 5]; // a vector of integers

    for num in numbers {
        println!("{}", num);
    }

    if is_even(result) {
        println!("{} is even.", result);
    } else {
        println!("{} is odd.", result);
    }

    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 5 {
            break;
        }
        println!("Looping... {}", counter);
    }
}

// A function that takes a string and prints a greeting
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// A function that adds two integers and returns the result
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// A function that checks if a number is even
fn is_even(num: i32) -> bool {
    num % 2 == 0
}


pub fn basics3(){
      // Constants
      const MAX_POINTS: u32 = 100_000;

      // Shadowing
      let x = 5;
      let x = x + 1;
      let x = x * 2;
      println!("The value of x is: {}", x); // x is now 12
  
      // Data types
      let guess: u32 = "42".parse().expect("Not a number!"); // scalar type
      let a = [1, 2, 3, 4, 5]; // array type
      let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"]; // array type
  
      // Functions with multiple parameters
      print_sum(5, 6);
  
      // Control flow
      let condition = true;
      let number = if condition { 5 } else { 6 };
      println!("The value of number is: {}", number);
  
      // Loop and while
      let mut counter = 0;
      let result = loop {
          counter += 1;
          if counter == 10 {
              break counter * 2;
          }
      };
      println!("The result is {}", result);
  
      let mut number = 3;
      while number != 0 {
          println!("{}!", number);
          number -= 1;
      }
      println!("LIFTOFF!!!");
  
      // For loop
      let a = [10, 20, 30, 40, 50];
      for element in a.iter() {
          println!("the value is: {}", element);
      }
  
      for number in (1..4).rev() {
          println!("{}!", number);
      }
      println!("LIFTOFF!!!");
  }
  
  fn print_sum(x: i32, y: i32) {
      println!("sum is: {}", x + y);
  }

  pub fn basics4(){
       // Structs
       let mut rect1 = Rectangle { width: 30, height: 50 };
       println!(
           "The area of the rectangle is {} square pixels.",
           rect1.area()
       );
       rect1.double_width();
       println!(
           "The area of the rectangle after doubling the width is {} square pixels.",
           rect1.area()
       );
   
       // Enums
       let home = IpAddr::V4(String::from("127.0.0.1"));
       let loopback = IpAddr::V6(String::from("::1"));
       home.call();
       loopback.call();
   
       let some_number = Some(5);
       let some_string = Some("a string");
       let absent_number: Option<i32> = None;
       println!("{:?}, {:?}, {:?}", some_number, some_string, absent_number);
   }
   
   // Structs
   struct Rectangle {
       width: u32,
       height: u32,
   }
   
   impl Rectangle {
       fn area(&self) -> u32 {
           self.width * self.height
       }
   
       fn double_width(&mut self) {
           self.width *= 2;
       }
   }
   
   // Enums
   enum IpAddr {
       V4(String),
       V6(String),
   }
   
   impl IpAddr {
       fn call(&self) {
           match self {
               IpAddr::V4(addr) => println!("Calling IPv4 address: {}", addr),
               IpAddr::V6(addr) => println!("Calling IPv6 address: {}", addr),
           }
       }
   }
  }




