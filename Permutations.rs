use std::io::{self, Write, BufWriter};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    if n == 1 {
        writeln!(writer, "1").unwrap();
    } else if n == 2 || n == 3 {
        writeln!(writer, "NO SOLUTION").unwrap();
    } else {
        // Print all even numbers first
        for i in (2..=n).step_by(2) {
            write!(writer, "{} ", i).unwrap();
        }
        // Print all odd numbers next
        for i in (1..=n).step_by(2) {
            write!(writer, "{} ", i).unwrap();
        }
    }
}
