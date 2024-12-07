use std::io::{stdin, Read};

use color_eyre::Result;

fn parse(s: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let (rules_section, updates_section) = s.split_once("\n\n").unwrap();
    let rules: Vec<(i32, i32)> = rules_section
        .lines()
        .map(|l| {
            let (a, b) = l.split_once('|').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();
    let updates: Vec<Vec<i32>> = updates_section
        .lines()
        .map(|l| l.split(',').map(|o| o.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let (rules, updates) = parse(&input);

    let mut ans = 0;
    for update in updates {
        if update.is_sorted_by(|a, b| !rules.contains(&(*b, *a))) {
            ans += update[update.len() / 2];
        }
    }

    println!("{}", ans);

    Ok(())
}
