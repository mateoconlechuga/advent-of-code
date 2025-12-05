use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed stdin");

    let parts: Vec<&str> = input.split("\n\n").collect();

    let ranges: Vec<_> = parts[0].lines()
        .map(|l| {
            let n: Vec<u64> = l.split('-').map(|x| x.parse().unwrap()).collect();
            n[0]..=n[1]
        })
        .collect();

    let nums: Vec<u64> = parts[1].lines().map(|l| l.parse().unwrap()).collect();

    println!("fresh: {}", nums.iter().filter(|&n| ranges.iter().any(|r| r.contains(n))).count());
}
