use std::io::{stdin, Read};

use color_eyre::Result;

fn get_problem_index(levels: &[i32]) -> Option<usize> {
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

fn is_safe_dampened(levels: &mut Vec<i32>) -> bool {
    if let Some(problem_idx) = get_problem_index(levels) {
        // Try the index, the following index, and the start (in case of misjudged direction of change)
        for i in [problem_idx, problem_idx + 1, 0] {
            // Try removing
            let elt = levels.remove(i);
            // Test
            let safe = get_problem_index(levels).is_none();

            if safe {
                return true;
            }

            // Restore for next check
            levels.insert(i, elt);
        }

        false
    } else {
        true
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut ans = 0;
    for line in input.lines() {
        let mut levels: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|o| o.parse::<i32>().unwrap())
            .collect();

        if is_safe_dampened(&mut levels) {
            ans += 1;
        }
    }

    println!("{}", ans);

    Ok(())
}
