use std::io::{stdin, Read};

use color_eyre::Result;

fn is_safe(levels: &[i32]) -> Option<usize> {
    let mut is_increasing = None;
    for (idx, x) in levels.windows(2).enumerate() {
        let a = x[0];
        let b = x[1];

        // First iteration
        if is_increasing.is_none() {
            is_increasing = Some(b > a);
        } else if is_increasing != Some(b > a) {
            return Some(idx);
        }

        let diff = (b - a).abs();
        if !(1..=3).contains(&diff) {
            return Some(idx);
        }
    }

    None
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

        if let Some(idx) = is_safe(&levels) {
            println!("possible danger with {:?}", &levels);

            let mut try_a = levels.clone();
            try_a.remove(idx);
            if is_safe(&try_a).is_none() {
                ans += 1;
                println!("qualified safe (1) {:?}", &try_a);
            } else {
                let mut try_b = levels.clone();
                try_b.remove(idx + 1);
                if is_safe(&try_b).is_none() {
                    ans += 1;
                    println!("qualified safe (2) {:?}", &try_b);
                } else {
                    let mut try_c = levels.clone();
                    try_c.remove(0);
                    if is_safe(&try_c).is_none() {
                        ans += 1;
                        println!("qualified safe (3) {:?}", &try_c);
                    }
                }
            }
        } else {
            ans += 1;
            println!("safe {:?}", &levels);
        }
    }

    println!("{}", ans);

    Ok(())
}
