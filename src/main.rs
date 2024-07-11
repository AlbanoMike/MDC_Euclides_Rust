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
