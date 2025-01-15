use std::io;


fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let mut number: usize  = input.trim().parse().expect("Invalid number");
    let mut sequence = vec![number];
    
    while number>1{
        if number %2 == 0{
            number  = number / 2;

        }else{
            number  = number * 3 + 1 ;
        }
        sequence.push(number);

        
    }
    println!("{} ",  sequence.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" "));
    
    

}