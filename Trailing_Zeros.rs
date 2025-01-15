use std::io;

fn main(){
    let mut input  = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let mut n:usize = input.trim().parse().unwrap();
    
    
    let mut result:usize =0;
    while n >= 5 {
        result += n/5;
        n /=5;

    }
    println!("{}",result);




}