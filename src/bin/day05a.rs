use std::{
    collections::HashMap,
    io::{stdin, Read},
};

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

    let (rules_vec, updates) = parse(&input);
    let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
    for rule in rules_vec {
        rules
            .entry(rule.0)
            .and_modify(|e| e.push(rule.1))
            .or_insert(vec![rule.1]);
    }

    let mut ans = 0;
    'updates: for update in updates {
        for i in 0..update.len() {
            let page = update[i];

            if let Some(afters) = rules.get(&page) {
                for after in afters {
                    if i > 0 && update[i - 1] == *after {
                        continue 'updates;
                    }
                }
            }
        }
        let mid = update[update.len() / 2];
        ans += mid;
    }

    println!("{}", ans);

    Ok(())
}
