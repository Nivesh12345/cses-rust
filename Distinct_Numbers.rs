use std::io;
use std::collections::HashSet;

fn main(){
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    // let n:usize = input.trim().parse().unwrap();
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let x = input.trim().split_whitespace().map(|n| n.parse().unwrap()).collect::<HashSet<usize>>();

    println!("{}", x.len());
}