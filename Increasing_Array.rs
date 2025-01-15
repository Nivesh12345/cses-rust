use std::io;

fn main(){
    let mut input  = String::new();

    io::stdin().read_line(&mut input).unwrap();
    
    let _n: usize = input.trim().parse().unwrap();
    
    input.clear();
    
    io::stdin().read_line(&mut input).unwrap();

    let elements:Vec<usize> = input.trim().split_whitespace().map(|n| n.parse().unwrap()).collect();

    
    // let mut prev = elements.next().unwrap();
    
    // println!("{:?}",elements);
    let mut prev = elements[0];
    let mut count = 0;

    for &ch in &elements[1..]{
        if ch < prev {
            count += prev - ch;

        }else {
            prev = ch;
        }
    }

    println!("{}",count);


}