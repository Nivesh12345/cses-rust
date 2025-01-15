use std::io::{self, Write, BufWriter};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i64 = input.trim().parse().unwrap();
    
    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());
    
    let total_sum = n * (n + 1) / 2;
    
    if total_sum % 2 != 0 {
        writeln!(writer, "NO").unwrap();
        return;
    }
    
    writeln!(writer, "YES").unwrap();
    let target_sum = total_sum / 2;
    let mut current_sum = 0;
    // let i: i64 = 1;
    let mut j: i64 = n;
    
    // Preallocating larger capacity to be safe
    let mut first_set = Vec::with_capacity(n as usize);
    let mut second_set = Vec::with_capacity(n as usize);
    
    // Loop adjusted for more clarity and correctness
    while current_sum != target_sum {
        if current_sum + j <= target_sum {
            first_set.push(j);
            current_sum += j;
        } else {
            second_set.push(j);
        }
        j -= 1;
    }
    
    // Adding the remaining elements to the second set
    while j > 0 {
        second_set.push(j);
        j -= 1;
    }
    
    // Printing results
    writeln!(writer, "{}\n", first_set.len()).unwrap();
    for num in &first_set {
        write!(writer, "{} ", num).unwrap();
    }
    writeln!(writer).unwrap();
    
    writeln!(writer, "{}\n", second_set.len()).unwrap();
    for num in &second_set {
        write!(writer, "{} ", num).unwrap();
    }
    writeln!(writer).unwrap();
}
