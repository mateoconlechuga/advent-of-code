use std::io::{self, Read};

fn is_invalid(id: u64) -> bool {
    let s: String = id.to_string();
    let len = s.len();

    for pattern_len in 1..=len/2 {
        if len % pattern_len == 0 {
            let num_repeats = len / pattern_len;
            let pattern = &s[0..pattern_len];
            
            let mut is_repeated = true;
            for i in 1..num_repeats {
                let start = i * pattern_len;
                let end = start + pattern_len;
                if &s[start..end] != pattern {
                    is_repeated = false;
                    break;
                }
            }

            if is_repeated && num_repeats == 2 {
                return true;
            }
        }
    }

    return false;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed stdin");
    let line = input.trim();

    let mut invalid_add: u64 = 0;

	for range in line.split(',') {
        let (low, high) = range.split_once('-').unwrap();
        let low_id = low.parse::<u64>().unwrap();
        let high_id = high.parse::<u64>().unwrap();

        for id in low_id..=high_id {
            if is_invalid(id) {
     	        //println!("{} + {}", invalid_add, id);
                invalid_add = invalid_add + id;
            }
        }
	}

    println!("Invalid ID addition: {}", invalid_add);
}
