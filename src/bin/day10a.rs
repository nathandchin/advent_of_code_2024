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
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    // fn debug(grid: &Vec<Vec<u32>>) {
    //     for row in grid {
    //         eprintln!("{:?}", row);
    //     }
    // }
    // debug(&grid);

    let mut roots = vec![];
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == 0 {
                roots.push(Node {
                    row: r,
                    col: c,
                    parent: None,
                });
            }
        }
    }

    let mut root_solns = vec![];
    for root in roots {
        root_solns.push(bfs(root, &grid));
    }

    // dbg!(root_solns.len());

    let mut ans = 0;

    for solns in root_solns {
        // for mut soln in solns {
        //     let mut vs = grid.clone();
        //     loop {
        //         if soln.parent.is_none() {
        //             break;
        //         }
        //         vs[soln.row][soln.col] = 100;
        //         soln = *soln.parent.unwrap();
        //     }

        //     for r in vs {
        //         for c in r {
        //             eprint!(
        //                 "{}",
        //                 if c == 100 {
        //                     "*".to_string()
        //                 } else {
        //                     c.to_string()
        //                 }
        //             )
        //         }
        //         eprintln!()
        //     }
        //     eprintln!();
        // }

        ans += solns.len();
    }

    // test_adjacent_edges();

    println!("{}", ans);

    Ok(())
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Node {
    row: usize,
    col: usize,
    parent: Option<Box<Node>>,
}

fn bfs(root: Node, grid: &Vec<Vec<u32>>) -> Vec<Node> {
    // fn bfs(root: Node, grid: &Vec<Vec<u32>>) -> Node {
    let mut queue = VecDeque::new();
    let mut explored = HashSet::new();
    let mut solutions = vec![];
    let mut tops = HashSet::new();
    queue.push_back(root);
    while !queue.is_empty() {
        // dbg!(&queue);
        let v = queue.pop_front().unwrap();
        // dbg!(&v);
        // eprintln!("here1");
        if grid[v.row][v.col] == 9 {
            // eprintln!("here2");
            // dbg!(explored.contains(&v));
            // return v;
            if !tops.contains(&(v.row, v.col)) {
                solutions.push(v.clone());
                tops.insert((v.row, v.col));
            }
            continue;
        }
        // eprintln!("here3");
        for mut w in adjacent_edges(v.clone(), grid) {
            // eprintln!("here4");
            if !explored.contains(&w) {
                // eprintln!("here5");
                w.parent = Some(Box::new(v.clone()));
                explored.insert(w.clone());
                queue.push_back(w);
            }
        }

        // std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    solutions
    // todo!();
}

fn adjacent_edges(v: Node, grid: &Vec<Vec<u32>>) -> Vec<Node> {
    // let xs = [-1, 0, 1, -1, 1, -1, 0, 1];
    // let ys = [-1, -1, -1, 0, 0, 1, 1, 1];
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

fn _test_adjacent_edges() {
    let v = Node {
        row: 1,
        col: 0,
        parent: None,
    };
    let grid = vec![
        vec![0, 1, 2, 3],
        vec![1, 2, 3, 4],
        vec![8, 7, 6, 5],
        vec![9, 8, 7, 6],
    ];
    // eprintln!("{:?}", adjacent_edges(v, &grid));
    dbg!(adjacent_edges(v, &grid));
}
