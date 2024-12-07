use std::{
    collections::HashSet,
    io::{stdin, Read},
};

use color_eyre::Result;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Guard {
    row: usize,
    col: usize,
    dir: Direction,
}

impl Guard {
    fn is_blocked(&self, grid: &[Vec<char>]) -> bool {
        assert!(!grid.is_empty());

        match self.dir {
            Direction::Up => self.row > 0 && grid[self.row - 1][self.col] == '#',
            Direction::Down => self.row < grid.len() - 1 && grid[self.row + 1][self.col] == '#',
            Direction::Left => self.col > 0 && grid[self.row][self.col - 1] == '#',
            Direction::Right => self.col < grid[0].len() - 1 && grid[self.row][self.col + 1] == '#',
        }
    }

    fn turn_right(&mut self) {
        self.dir = match self.dir {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn go_forward(&mut self, grid: &[Vec<char>]) -> Option<()> {
        assert!(!grid.is_empty());

        match self.dir {
            Direction::Up => {
                if self.row == 0 {
                    return None;
                } else {
                    self.row -= 1
                }
            }
            Direction::Down => {
                if self.row == grid.len() - 1 {
                    return None;
                } else {
                    self.row += 1
                }
            }
            Direction::Left => {
                if self.col == 0 {
                    return None;
                } else {
                    self.col -= 1
                }
            }
            Direction::Right => {
                if self.row == grid[0].len() - 1 {
                    return None;
                } else {
                    self.col += 1
                }
            }
        }

        Some(())
    }

    fn advance(&mut self, grid: &[Vec<char>]) -> Option<()> {
        if self.is_blocked(grid) {
            self.turn_right();
            Some(())
        } else {
            self.go_forward(grid)
        }
    }
}

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let guard_pos = grid
        .iter()
        .enumerate()
        .find_map(|(row_idx, s)| {
            if let Some((col_idx, _)) = s.iter().enumerate().find(|(_, &c)| c == '^') {
                Some((row_idx, col_idx))
            } else {
                None
            }
        })
        .unwrap();
    let mut guard = Guard {
        row: guard_pos.0,
        col: guard_pos.1,
        dir: Direction::Up,
    };

    let mut visited = HashSet::new();
    visited.insert(guard.clone());

    loop {
        guard.advance(&grid);
        if visited.contains(&guard) {
            break;
        } else {
            visited.insert(guard.clone());
        }
    }

    let unique_positions: HashSet<(usize, usize)> =
        visited.into_iter().map(|o| (o.row, o.col)).collect();
    println!("{}", unique_positions.len());

    Ok(())
}
