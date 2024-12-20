use std::{
    collections::{HashMap, HashSet},
    io::{stdin, Read},
};

use color_eyre::Result;
use itertools::Itertools;

type Point = (usize, usize);

fn find_antinodes(positions: &[Point]) -> Vec<Point> {
    let mut res = vec![];
    for perm in positions.iter().permutations(2) {
        let a = perm[0];
        let b = perm[1];

        let diff = (b.0 as i32 - a.0 as i32, b.1 as i32 - a.1 as i32);

        let new = if let (Some(r), Some(c)) = (
            b.0.checked_add_signed(diff.0 as isize),
            b.1.checked_add_signed(diff.1 as isize),
        ) {
            (r, c)
        } else {
            continue;
        };

        res.push(new);
    }

    res
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut freqs: HashMap<char, Vec<Point>> = HashMap::new();
    let lines: Vec<_> = input.lines().collect();
    let num_rows = lines.len();
    let num_cols = lines[0].len();

    for (row, line) in lines.into_iter().enumerate() {
        for (col, ch) in line.char_indices() {
            if ch.is_ascii_alphanumeric() {
                freqs
                    .entry(ch)
                    .and_modify(|v| v.push((row, col)))
                    .or_insert(vec![(row, col)]);
            }
        }
    }

    let mut antinodes: HashSet<Point> = HashSet::new();
    for freq in freqs.keys() {
        for pos in find_antinodes(&freqs[freq])
            .into_iter()
            .filter(|(r, c)| r < &num_rows && c < &num_cols)
        {
            antinodes.insert(pos);
        }
    }
    println!("{}", antinodes.len());

    Ok(())
}
