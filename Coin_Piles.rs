use std::io::{self,BufRead, Write};


fn main(){

    
    let mut input = io::stdin().lock().lines();

    let mut output = io::BufWriter::new(io::stdout());

    
    let n:usize = input.next().unwrap().unwrap().trim().parse().unwrap();
    
    
    
    for _ in 0..n{
        let line  = input.next().unwrap().unwrap();


        
        let mut nums= line.split_whitespace().map(|n| n.parse::<usize>().unwrap());

        let (a,b) = (nums.next().unwrap(),nums.next().unwrap());

           
        
        
        if (a+b) %3 == 0 && 2*a>=b && 2*b >=a {
        
            writeln!(output,"YES").unwrap();
        }else{
            
            writeln!(output,"NO").unwrap();
            
        }

        
    }
}