use std::{
    collections::HashMap,
    io::{stdin, Read},
};

use color_eyre::Result;

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut lefts = vec![];
    let mut rights = HashMap::new();

    for line in input.lines() {
        let parts: Vec<_> = line.split_ascii_whitespace().collect();
        let left: i64 = parts[0].parse()?;
        let right: i64 = parts[1].parse()?;
        lefts.push(left);
        rights.entry(right).and_modify(|o| *o += 1).or_insert(1);
    }

    let mut ans = 0;

    for left in lefts {
        let sim = left * rights.get(&left).or(Some(&0)).unwrap();
        ans += sim;
    }

    println!("{}", ans);

    Ok(())
}
