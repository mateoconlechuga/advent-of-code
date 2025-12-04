use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let grid: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();

    let mut total = 0;
    let (h, w) = (grid.len(), grid[0].len());

    for r in 0..h {
        for c in 0..w {
            if grid[r][c] != '@' {
                continue;
            }

            let mut cnt = 0;
            for (dr, dc) in [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)] {
                if let (Some(nr), Some(nc)) = (r.checked_add_signed(dr), c.checked_add_signed(dc)) {
                    if nr < h && nc < w && grid[nr][nc] == '@' {
                        cnt += 1;
                    }
                }
            }

            if cnt < 4 {
                total += 1;
            }
        }
    }

    println!("{}", total);
}
