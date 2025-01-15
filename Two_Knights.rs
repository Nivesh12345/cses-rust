use std::io::{self,Write,BufWriter};

fn main(){
    let mut n = String::new();
    io::stdin().read_line(&mut n).unwrap();
    let n:usize = n.trim().parse().unwrap();

    let stdout = io::stdout();
    let mut output = BufWriter::new(stdout.lock());

    for k in 1..=n {
        let total_placements = k * k * (k * k -1) /2;
    
        let attack_placements =  if k >= 3 {
            4 * (k-1) *(k-2)
            
        }else {0};

        writeln!(output,"{}",total_placements - attack_placements).unwrap();

    }
    



   
}