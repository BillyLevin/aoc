use std::time::Instant;

fn main() {
    let input = include_str!("../input.txt");

    let start = Instant::now();
    let answer = part_one(input);
    let elapsed = start.elapsed();
    println!("Part one: {}, took {}ms", answer, elapsed.as_millis());

    let start = Instant::now();
    let answer = part_two(input);
    let elapsed = start.elapsed();
    println!("Part two: {}, took {}ms", answer, elapsed.as_millis());
}

#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Roll,
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

fn part_one(input: &str) -> u64 {
    let grid: Vec<Vec<Cell>> = input
        .lines()
        .map(|row| {
            row.chars()
                .map(|ch| match ch {
                    '@' => Cell::Roll,
                    '.' => Cell::Empty,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut result = 0;

    for row_index in 0..rows {
        for col_index in 0..cols {
            if grid[row_index][col_index] == Cell::Roll {
                let mut adjacent_rolls = 0;
                for (row_offset, col_offset) in DIRECTIONS {
                    let new_row = row_index as isize + row_offset;
                    if new_row < 0 {
                        continue;
                    }

                    let new_col = col_index as isize + col_offset;
                    if new_col < 0 {
                        continue;
                    }

                    if let Some(row) = grid.get(new_row as usize)
                        && let Some(cell) = row.get(new_col as usize)
                    {
                        match cell {
                            Cell::Empty => {}
                            Cell::Roll => {
                                adjacent_rolls += 1;
                            }
                        }
                    }
                }
                if adjacent_rolls < 4 {
                    result += 1;
                }
            }
        }
    }

    result
}

fn part_two(input: &str) -> u64 {
    let mut grid: Vec<Vec<Cell>> = input
        .lines()
        .map(|row| {
            row.chars()
                .map(|ch| match ch {
                    '@' => Cell::Roll,
                    '.' => Cell::Empty,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut result = 0;

    loop {
        let mut stop = true;

        for row_index in 0..rows {
            for col_index in 0..cols {
                if grid[row_index][col_index] == Cell::Roll {
                    let mut adjacent_rolls = 0;
                    for (row_offset, col_offset) in DIRECTIONS {
                        let new_row = row_index as isize + row_offset;
                        if new_row < 0 {
                            continue;
                        }

                        let new_col = col_index as isize + col_offset;
                        if new_col < 0 {
                            continue;
                        }

                        if let Some(row) = grid.get(new_row as usize)
                            && let Some(cell) = row.get(new_col as usize)
                        {
                            match cell {
                                Cell::Empty => {}
                                Cell::Roll => {
                                    adjacent_rolls += 1;
                                }
                            }
                        }
                    }

                    if adjacent_rolls < 4 {
                        stop = false;
                        grid[row_index][col_index] = Cell::Empty;
                        result += 1;
                    }
                }
            }
        }

        if stop {
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_four_part_one() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        assert_eq!(part_one(input), 13);
    }

    #[test]
    fn day_four_part_two() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

        assert_eq!(part_two(input), 43);
    }
}
