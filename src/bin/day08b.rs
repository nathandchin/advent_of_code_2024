use std::{
    collections::{HashMap, HashSet},
    io::{stdin, Read},
};

use color_eyre::Result;
use itertools::Itertools;

type Point = (usize, usize);

fn find_antinodes(positions: &[Point], num_rows: usize, num_cols: usize) -> Vec<Point> {
    let mut res = vec![];
    for perm in positions.iter().permutations(2) {
        let a = perm[0];
        let b = perm[1];
        let mut cur = b.clone();

        let diff = (b.0 as i32 - a.0 as i32, b.1 as i32 - a.1 as i32);

        while let (Some(r), Some(c)) = (
            cur.0.checked_add_signed(diff.0 as isize),
            cur.1.checked_add_signed(diff.1 as isize),
        ) {
            let new = (r, c);
            if new.0 >= num_rows || new.1 >= num_cols {
                break;
            }
            eprintln!("{:?}", &new);
            res.push(new);
            cur = new;
        }
    }

    res
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let mut freqs: HashMap<char, Vec<Point>> = HashMap::new();
    let lines: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let num_rows = lines.len();
    let num_cols = lines[0].len();

    let mut antennas = 0;
    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.into_iter().enumerate() {
            if ch.is_ascii_alphanumeric() {
                freqs
                    .entry(*ch)
                    .and_modify(|v| v.push((row, col)))
                    .or_insert(vec![(row, col)]);
                antennas += 1;
            }
        }
    }

    let mut already = 0;
    let mut antinodes: HashSet<Point> = HashSet::new();
    for freq in freqs.keys() {
        for pos in find_antinodes(&freqs[freq], num_rows, num_cols).into_iter() {
            if lines[pos.0][pos.1].is_alphanumeric() {
            // if &lines[pos.0][pos.1] == freq {
            eprintln!("already: {:?}", pos);
                already += 1;
            }
            antinodes.insert(pos);
        }
        eprintln!();
    }
    dbg!(antinodes.len() + antennas);
    dbg!(already);
    println!("{}", antinodes.len() + antennas - already);

    Ok(())
}
