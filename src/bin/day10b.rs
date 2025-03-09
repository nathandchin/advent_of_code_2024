use std::{
    collections::{HashSet, VecDeque},
    io::{stdin, Read},
};

use color_eyre::Result;

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|l| {
            l.chars()
                // Default to 100 as a sentinel value. Some test inputs have
                // non-digit chars, so this protects against that.
                .map(|c| c.to_digit(10).unwrap_or(100))
                .collect()
        })
        .collect();

    let mut roots = vec![];
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == 0 {
                roots.push(Node {
                    row,
                    col,
                    parent: None,
                });
            }
        }
    }

    let mut solutions = vec![];
    for root in roots {
        solutions.push(bfs(root, &grid));
    }

    let ans = solutions.iter().fold(0, |acc, s| acc + s.len());

    println!("{}", ans);

    Ok(())
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Node {
    row: usize,
    col: usize,
    // Reverse linked list
    parent: Option<Box<Node>>,
}

fn bfs(root: Node, grid: &[Vec<u32>]) -> Vec<Node> {
    let mut queue = VecDeque::new();
    let mut explored = HashSet::new();
    // Paths
    let mut solutions = vec![];

    queue.push_back(root);
    while !queue.is_empty() {
        let v = queue.pop_front().unwrap();
        if grid[v.row][v.col] == 9 {
            if !solutions.contains(&v) {
                solutions.push(v);
            }
            continue;
        }
        for mut w in adjacent_edges(v.clone(), grid) {
            if !explored.contains(&w) {
                w.parent = Some(Box::new(v.clone()));
                explored.insert(w.clone());
                queue.push_back(w);
            }
        }
    }

    solutions
}

fn adjacent_edges(v: Node, grid: &[Vec<u32>]) -> Vec<Node> {
    let xs = [0, -1, 1, 0];
    let ys = [-1, 0, 0, 1];

    let mut combos = vec![];
    for i in 0..xs.len() {
        let dx = xs[i];
        let dy = ys[i];
        let x = v.col as i32 + dx;
        let y = v.row as i32 + dy;

        if 0 <= y
            && y < grid.len() as i32
            && 0 <= x
            && x < grid[0].len() as i32
            // Smooth path
            && grid[y as usize][x as usize] == grid[v.row][v.col] + 1
        {
            combos.push(Node {
                row: y as usize,
                col: x as usize,
                parent: None,
            });
        }
    }
    combos
}
