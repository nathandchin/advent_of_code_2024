use std::io::{stdin, Read};

use color_eyre::Result;

fn is_safe(levels: &[i32]) -> bool {
    let mut is_increasing = None;
    for x in levels.windows(2) {
        let a = x[0];
        let b = x[1];

        // First iteration
        if is_increasing.is_none() {
            is_increasing = Some(b > a);
        } else if is_increasing != Some(b > a) {
            return false;
        }

        let diff = (b - a).abs();
        if !(1..=3).contains(&diff) {
            return false;
        }
    }

    true
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut ans = 0;
    for line in input.lines() {
        let levels: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|o| o.parse::<i32>().unwrap())
            .collect();

        if is_safe(&levels) {
            ans += 1;
        }
    }

    println!("{}", ans);

    Ok(())
}
