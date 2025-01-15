use std::io::{self, Write, BufWriter};

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Please enter a valid number");

    let stdout = io::stdout();
    let mut handle = BufWriter::new(stdout.lock());

    for k in 1..=n {
        // Calculate the total placements and attacking placements
        let total_placements = k * k * (k * k - 1) / 2;
        let attacking_placements = if k >= 3 { 4 * (k - 1) * (k - 2) } else { 0 };

        // Directly write the result to the buffered output
        writeln!(handle, "{}", total_placements - attacking_placements).expect("Failed to write to stdout");
    }
}
