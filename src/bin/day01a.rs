use std::io::{stdin, Read};

use color_eyre::Result;

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut lefts = vec![];
    let mut rights = vec![];

    for line in input.lines() {
        let parts: Vec<_> = line.split_ascii_whitespace().collect();
        lefts.push(parts[0].parse::<i64>()?);
        rights.push(parts[1].parse::<i64>()?);
    }

    lefts.sort();
    rights.sort();

    let mut ans = 0;

    for (left, right) in lefts.into_iter().zip(rights) {
        ans += (left - right).abs();
    }

    println!("{}", ans);

    Ok(())
}
