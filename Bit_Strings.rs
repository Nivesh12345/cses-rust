use std::io;

const MODULUS:usize = 10_usize.pow(9)+7;
fn main() {
    let mut input  = String::new();

    io::stdin().read_line(&mut input).unwrap();
    
    let n:usize = input.trim().parse().unwrap();
    
    let mut result:usize = 2;
    for _i in 1..n{
        
        result = (result*2) % MODULUS; 
    }

    println!("{}",result);


}