use std::io;

fn main(){
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    // std::io::stdin().read_line(&mut input).prase().unwrap();
    
    let n: usize = input.trim().parse().unwrap();
    
    let sum_all  = n * ( n + 1 ) / 2;
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    // std::io::stdin().read_line(&mut input).prase().unwrap();

    // let numbers = input.trim().split_whitespace().map(|n| n.prase().unwrap()).collect()::<Vec<usize>>;
    let numbers:Vec<usize> = input.trim().split_whitespace().map(|n| n.parse().unwrap()).collect();

    let sum_given  = numbers.iter().sum::<usize>();

    let missing_number = sum_all - sum_given;

    println!("{}",missing_number);
}
