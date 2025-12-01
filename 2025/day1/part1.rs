use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read from stdin");
    
    let mut counter = 50;
    let mut zero_hits = 0;
    
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() { continue; }
        
        let (dir, num) = line.split_at(1);
        let n = num.parse::<i32>().unwrap();
        
        counter = match dir {
            "L" => ((counter - n) % 100 + 100) % 100,
            "R" => (counter + n) % 100,
            _ => counter,
        };
        
        if counter == 0 { zero_hits += 1; }
        println!("{} -> counter: {}", line, counter);
    }
    
    println!("\nFinal counter: {}", counter);
    println!("Times zero was hit: {}", zero_hits);
}
