use std::io::{stdin, Read};

use color_eyre::Result;

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut lefts = vec![];
    let mut rights = vec![];

    for line in input.lines() {
        let parts: Vec<_> = line.split_ascii_whitespace().collect();
        let left: i64 = parts[0].parse()?;
        let right: i64 = parts[1].parse()?;
        lefts.push(left);
        rights.push(right);
    }

    lefts.sort();
    rights.sort();

    let mut ans = 0;

    for (left, right) in lefts.into_iter().zip(rights) {
        let dist = (left - right).abs();
        ans += dist;
    }

    println!("{}", ans);

    Ok(())
}
