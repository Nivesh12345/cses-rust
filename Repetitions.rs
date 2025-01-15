// use std::io;
fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}
fn main() {
    let dna_sequence = read_input();
    
    let mut chars = dna_sequence.chars().peekable();
    let mut prev =  chars.next().unwrap();

    let mut longest = 1;
    let mut current_length = 1;
  
    
    for ch in chars {
        if ch == prev {
            current_length += 1;
        } else {
            longest = longest.max(current_length);
            current_length = 1;
        }
        prev = ch;
    }
    longest = longest.max(current_length);

    println!("{}", longest);
}