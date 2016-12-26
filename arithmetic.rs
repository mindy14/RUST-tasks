use std::{i8,i16,i32,i64,u8,u16,u32,u64,f32,f64,isize,usize};
use std::io::stdin;

fn main() {
	println!("Rust Program to perform Arithmetic Functions!");

	println!("Enter the first number: ");
	let mut num1 = String::new();

        io::stdin().read_line(&mut num1)
            .expect("Error: Line not read");

    println!("Enter the second number: ");
	let mut num2 = String::new();

        io::stdin().read_line(&mut num2)
            .expect("Error: Line not read");

     println!("Addition: {}", num1+num2);
     println!("Subtraction: {}", num1-num2);
     println!("Multiplication: {}", num1*num2);
     println!("Division: {}", num1/num2);
}