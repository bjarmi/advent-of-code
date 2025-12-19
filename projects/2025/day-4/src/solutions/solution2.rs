use crate::common::parse_grid::parse_grid;

pub fn solve_part2(input: &[String]) -> i32 {
    let mut grid: Vec<Vec<bool>> = parse_grid(input);
    let mut pickable_rolls = 0;

    let total_rows = grid.len() as isize;
    let total_columns = grid[0].len() as isize;

    let mut removed_roll = true;

    while removed_roll {
        removed_roll = false;

        for y in 0..total_rows {
            for x in 0..total_columns {
                if !grid[y as usize][x as usize] {
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
                if left >= 0 && grid[y as usize][left as usize] {
                    occupied_count += 1;
                }
                if right < total_columns && grid[y as usize][right as usize] {
                    occupied_count += 1;
                }
                if left >= 0 && bottom < total_rows && grid[bottom as usize][left as usize] {
                    occupied_count += 1;
                }
                if bottom < total_rows && grid[bottom as usize][x as usize] {
                    occupied_count += 1;
                }
                if right < total_columns
                    && bottom < total_rows
                    && grid[bottom as usize][right as usize]
                {
                    occupied_count += 1;
                }

                if occupied_count < 4 {
                    grid[y as usize][x as usize] = false;
                    removed_roll = true;
                    pickable_rolls += 1;
                }
            }
        }
    }

    pickable_rolls
}
