
use core::num;
use std::{f32::DIGITS, io, str::Chars};
fn convert_to_int(data: &str) -> i32{
  let x = data.trim().parse::<i32>().unwrap();
  x
}

fn main() {
 let mut a = 16;
 let mut b = 12;
 let frase = format!("o maior divisor entre {} e {} é",a,b);
 
 while b != 0 {
     let temp = b;
     b = a % b ;
     a = temp;
 }
 println!("{} {}",frase,a);
}