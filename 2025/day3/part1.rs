use std::io::{self, BufRead};

fn find_combo(s: &str, k: usize) -> u64 {
    let bytes = s.as_bytes();
    let mut start = 0;
    let mut result = 0;
    
    for i in 0..k.min(bytes.len()) {
        let remaining = k - i - 1;
        let end = bytes.len() - remaining;

        let &max_digit = bytes[start..end].iter().max().unwrap();
        let offset = bytes[start..end].iter().position(|&d| d == max_digit).unwrap();
        
        result = result * 10 + (max_digit - b'0') as u64;
        start += offset + 1;
    }
    
    result
}

fn main() {
    let mut result: u64 = 0;

    for line in io::stdin().lock().lines().map(|l| l.unwrap()) {
        let largest: u64 = find_combo(&line, 2);
        result += largest;
        println!("largest: {}", largest);
    }
    
    println!("result: {}", result);
}
