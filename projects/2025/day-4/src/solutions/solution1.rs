use crate::common::parse_grid::parse_grid;

pub fn solve_part1(input: &[String]) -> i32 {
    let grid = parse_grid(input);
    let mut pickable_rolls = 0;

    let total_rows = grid.len() as isize;
    let total_columns = grid[0].len() as isize;

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if !*cell {
                continue;
            }
            let mut occupied_count = 0;

            let left = x as isize - 1;
            let right = x as isize + 1;
            let top = y as isize - 1;
            let bottom = y as isize + 1;

            if left >= 0 && top >= 0 && grid[top as usize][left as usize] {
                occupied_count += 1;
            }
            if top >= 0 && grid[top as usize][x as usize] {
                occupied_count += 1;
            }
            if right < total_columns && top >= 0 && grid[top as usize][right as usize] {
                occupied_count += 1;
            }
            if left >= 0 && grid[y][left as usize] {
                occupied_count += 1;
            }
            if right < total_columns && grid[y][right as usize] {
                occupied_count += 1;
            }
            if left >= 0 && bottom < total_rows && grid[bottom as usize][left as usize] {
                occupied_count += 1;
            }
            if bottom < total_rows && grid[bottom as usize][x as usize] {
                occupied_count += 1;
            }
            if right < total_columns && bottom < total_rows && grid[bottom as usize][right as usize]
            {
                occupied_count += 1;
            }

            if occupied_count < 4 {
                pickable_rolls += 1;
            }
        }
    }

    pickable_rolls
}
