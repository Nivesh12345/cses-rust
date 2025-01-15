use std::io;

fn main(){
    let mut input  = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let s = input.trim().to_string();

    
    let mut freq = [0;26];
    
    for c in s.chars(){
        freq[(c as u8 - b'A') as usize] +=1;

    }

    

    let mut odd_count = 0;
    let mut odd_char = None;
    let mut left_half = String::new();

    for (i , &count) in freq.iter().enumerate() {
        let ch = (b'A' + i as u8) as char;

        if count % 2 != 0 {
            odd_count+= 1;

            if odd_count > 1{
                println!("NO SOLUTION");
                return;
            }
            odd_char = Some(ch);
        }
        left_half.push_str(&ch.to_string().repeat(count/2));
    }

    let mut result = left_half.clone();

    if let Some(ch) = odd_char {
        result.push(ch);
    }

    result.push_str(&left_half.chars().rev().collect::<String>());

    println!("{}", result);

}