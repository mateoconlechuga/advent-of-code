use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed stdin");

    let mut ingredients = Vec::new();

    for line in input.lines().take_while(|l| !l.is_empty()) {
        let n: Vec<u64> = line.split('-').map(|x| x.parse().unwrap()).collect();
        ingredients.push((n[0], 1));
        ingredients.push((n[1] + 1, -1));
    }

    ingredients.sort_unstable();

    let mut count = 0;
    let mut active = 0;
    let mut prev = 0;

    for (pos, delta) in ingredients {
        if active > 0 {
            count += pos - prev;
        }
        active += delta;
        prev = pos;
    }

    println!("{}", count);
}
