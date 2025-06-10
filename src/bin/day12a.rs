use std::{
    collections::{HashSet, VecDeque},
    io::{stdin, Read},
};

use color_eyre::Result;

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let height = grid.len();
    let width = grid[0].len();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut ans = 0;

    for (row, line) in grid.iter().enumerate() {
        for (col, &ch) in line.iter().enumerate() {
            let mut perimeter = 0;
            let mut area = 0;

            let mut adjoining = VecDeque::new();
            adjoining.push_back((row, col));

            while let Some((row, col)) = adjoining.pop_front() {
                if visited.contains(&(row, col)) {
                    continue;
                }

                if row == 0 || grid[row - 1][col] != ch {
                    perimeter += 1;
                } else {
                    adjoining.push_back((row - 1, col));
                }
                if row == height - 1 || grid[row + 1][col] != ch {
                    perimeter += 1;
                } else {
                    adjoining.push_back((row + 1, col));
                }

                if col == 0 || grid[row][col - 1] != ch {
                    perimeter += 1
                } else {
                    adjoining.push_back((row, col - 1));
                }
                if col == width - 1 || grid[row][col + 1] != ch {
                    perimeter += 1
                } else {
                    adjoining.push_back((row, col + 1));
                }

                visited.insert((row, col));
                area += 1;
            }
            ans += perimeter * area;
        }
    }

    println!("{}", ans);

    Ok(())
}
