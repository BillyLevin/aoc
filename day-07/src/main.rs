use std::{collections::HashMap, fmt::Display, time::Instant};

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

#[derive(Debug)]
struct Grid {
    cells: Vec<Vec<char>>,
    start: Position,
    dimensions: Dimensions,
}

impl Grid {
    fn new(input: &str) -> Self {
        let cells: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();

        let start = cells
            .iter()
            .enumerate()
            .find_map(|(row_index, row)| {
                let col = row.iter().position(|cell| *cell == 'S')?;

                Some(Position {
                    top: row_index,
                    left: col,
                })
            })
            .unwrap();

        let width = cells[0].len();
        let height = cells.len();

        Self {
            cells,
            start,
            dimensions: Dimensions { width, height },
        }
    }

    fn count_splits(&mut self) -> u64 {
        let mut beams = vec![self.start];
        let mut splits = 0;

        while let Some(beam) = beams.pop() {
            let Some(next) = self.down(beam) else {
                continue;
            };

            match self.cell(next) {
                '.' => {
                    *self.cell_mut(next) = '|';
                    beams.push(next);
                }
                '^' => {
                    let split_beams = self.split_beam(next);
                    for split in &split_beams {
                        *self.cell_mut(*split) = '|';
                    }
                    beams.extend_from_slice(&split_beams);
                    splits += 1;
                }
                '|' => {}
                _ => unreachable!(),
            };
        }

        splits
    }

    fn count_timelines(&self) -> u64 {
        let mut cache = HashMap::new();
        1 + self.timelines(self.start, &mut cache)
    }

    fn timelines(&self, start: Position, cache: &mut HashMap<Position, u64>) -> u64 {
        if let Some(&result) = cache.get(&start) {
            return result;
        }

        let mut beams = vec![start];
        let mut timelines = 0;

        while let Some(beam) = beams.pop() {
            let Some(next) = self.down(beam) else {
                continue;
            };

            match self.cell(next) {
                '.' => {
                    beams.push(next);
                }
                '^' => {
                    let split_beams = self.split_beam(next);

                    match split_beams.as_slice() {
                        [] => {}
                        [split] => {
                            timelines += self.timelines(*split, cache);
                        }
                        [left_split, right_split] => {
                            timelines += 1;
                            timelines += self.timelines(*right_split, cache);
                            timelines += self.timelines(*left_split, cache);
                        }
                        _ => unreachable!(),
                    }
                }
                _ => unreachable!(),
            };
        }

        cache.insert(start, timelines);
        timelines
    }

    fn down(&self, position: Position) -> Option<Position> {
        let top = position.top + 1;
        if top >= self.dimensions.height {
            return None;
        }

        Some(Position {
            top,
            left: position.left,
        })
    }

    fn cell(&self, position: Position) -> char {
        self.cells[position.top][position.left]
    }

    fn cell_mut(&mut self, position: Position) -> &mut char {
        &mut self.cells[position.top][position.left]
    }

    fn split_beam(&self, position: Position) -> Vec<Position> {
        match (self.left(position), self.right(position)) {
            (None, None) => vec![],
            (None, Some(right_beam)) => vec![right_beam],
            (Some(left_beam), None) => vec![left_beam],
            (Some(left_beam), Some(right_beam)) => vec![left_beam, right_beam],
        }
    }

    fn left(&self, position: Position) -> Option<Position> {
        let left = position.left as isize - 1;
        if left < 0 {
            return None;
        }

        Some(Position {
            top: position.top,
            left: left as usize,
        })
    }

    fn right(&self, position: Position) -> Option<Position> {
        let left = position.left + 1;
        if left >= self.dimensions.width {
            return None;
        }

        Some(Position {
            top: position.top,
            left,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    top: usize,
    left: usize,
}

#[derive(Debug, Clone, Copy)]
struct Dimensions {
    width: usize,
    height: usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for col in row {
                write!(f, "{col}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn part_one(input: &str) -> u64 {
    let mut grid = Grid::new(input);
    grid.count_splits()
}

fn part_two(input: &str) -> u64 {
    let grid = Grid::new(input);
    grid.count_timelines()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_seven_part_one() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(part_one(input), 21);
    }

    #[test]
    fn day_seven_part_two() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(part_two(input), 40);
    }
}
