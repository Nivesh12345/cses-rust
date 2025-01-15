use std::io::{self, BufWriter, Write};

fn main(){
    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap();

    let n:usize = input.trim().parse().unwrap();

    let stdout = io::stdout();

    let mut writer = BufWriter::new(stdout.lock());

    let num_codes = 1 << n;
    // println!("{}",num_codes);

    for i in 0..num_codes {
        let gray_code = i^(i>>1);

        let binary = format!("{:0width$b}",gray_code,width = n);

        writeln!(writer, "{}", binary).unwrap();
    }

    
}