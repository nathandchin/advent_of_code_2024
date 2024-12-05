use std::io::{stdin, Read};

use color_eyre::Result;

fn main() -> Result<()> {
    let mut input = String::new();
    stdin().read_to_string(&mut input)?;

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut ans = 0;
    for (row_idx, row) in input.lines().enumerate() {
        for (col_idx, _col) in row.char_indices() {
            if col_idx < cols - 3
                && (&row[col_idx..col_idx + 4] == "XMAS" || &row[col_idx..col_idx + 4] == "SAMX")
            {
                ans += 1;
            }
            if row_idx < rows - 3
                && grid[row_idx][col_idx] == 'X'
                && grid[row_idx + 1][col_idx] == 'M'
                && grid[row_idx + 2][col_idx] == 'A'
                && grid[row_idx + 3][col_idx] == 'S'
            {
                ans += 1;
            }
            if row_idx < rows - 3
                && grid[row_idx][col_idx] == 'S'
                && grid[row_idx + 1][col_idx] == 'A'
                && grid[row_idx + 2][col_idx] == 'M'
                && grid[row_idx + 3][col_idx] == 'X'
            {
                ans += 1;
            }
            if row_idx < rows - 3
                && col_idx < cols - 3
                && grid[row_idx][col_idx] == 'X'
                && grid[row_idx + 1][col_idx + 1] == 'M'
                && grid[row_idx + 2][col_idx + 2] == 'A'
                && grid[row_idx + 3][col_idx + 3] == 'S'
            {
                ans += 1;
            }
            if row_idx < rows - 3
                && col_idx < cols - 3
                && grid[row_idx][col_idx] == 'S'
                && grid[row_idx + 1][col_idx + 1] == 'A'
                && grid[row_idx + 2][col_idx + 2] == 'M'
                && grid[row_idx + 3][col_idx + 3] == 'X'
            {
                ans += 1;
            }
            if row_idx > 2
                && col_idx < cols - 3
                && grid[row_idx][col_idx] == 'X'
                && grid[row_idx - 1][col_idx + 1] == 'M'
                && grid[row_idx - 2][col_idx + 2] == 'A'
                && grid[row_idx - 3][col_idx + 3] == 'S'
            {
                ans += 1;
            }
            if row_idx > 2
                && col_idx < cols - 3
                && grid[row_idx][col_idx] == 'S'
                && grid[row_idx - 1][col_idx + 1] == 'A'
                && grid[row_idx - 2][col_idx + 2] == 'M'
                && grid[row_idx - 3][col_idx + 3] == 'X'
            {
                ans += 1;
            }
        }
    }

    println!("{}", ans);

    Ok(())
}
