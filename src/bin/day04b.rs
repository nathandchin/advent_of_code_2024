use std::io::{stdin, Read};

use color_eyre::Result;

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = grid.len();

    let mut ans = 0;
    for (row_idx, row) in input.lines().enumerate() {
        for (col_idx, _col) in row.char_indices() {
            if grid[row_idx][col_idx] != 'A'
                || row_idx < 1
                || row_idx > rows - 2
                || col_idx < 1
                || col_idx > rows - 2
            {
                continue;
            }

            if ((grid[row_idx - 1][col_idx - 1] == 'M' && grid[row_idx + 1][col_idx + 1] == 'S')
                || (grid[row_idx - 1][col_idx - 1] == 'S' && grid[row_idx + 1][col_idx + 1] == 'M'))
                && ((grid[row_idx - 1][col_idx + 1] == 'M'
                    && grid[row_idx + 1][col_idx - 1] == 'S')
                    || (grid[row_idx - 1][col_idx + 1] == 'S'
                        && grid[row_idx + 1][col_idx - 1] == 'M'))
            {
                ans += 1;
            }
        }
    }

    println!("{}", ans);

    Ok(())
}
