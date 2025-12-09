use std::{cmp, collections::HashMap, time::Instant};

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");

    let start = Instant::now();
    let answer = part_one(input);
    let elapsed = start.elapsed();
    println!("Part one: {answer}, took {elapsed:?}");

    let start = Instant::now();
    let answer = part_two(input);
    let elapsed = start.elapsed();
    println!("Part two: {answer}, took {elapsed:?}");
}

#[derive(Debug, Clone, Copy)]
struct Position {
    left: usize,
    top: usize,
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        let (left, top) = value.split_once(',').unwrap();

        Self {
            left: left.parse().unwrap(),
            top: top.parse().unwrap(),
        }
    }
}

fn area(pos1: &Position, pos2: &Position) -> usize {
    (pos1.left.abs_diff(pos2.left) + 1) * (pos1.top.abs_diff(pos2.top) + 1)
}

fn part_one(input: &str) -> usize {
    let positions: Vec<Position> = input.lines().map(Position::from).collect();

    positions
        .iter()
        .enumerate()
        .flat_map(|(i, pos)| positions.iter().skip(i + 1).map(|pos2| area(pos, pos2)))
        .max()
        .unwrap()
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Boundary,
    Inside,
    Outside,
}

#[derive(Debug)]
struct Floor {
    tiles: Vec<Vec<Tile>>,
    compressed_xs: HashMap<usize, usize>,
    compressed_ys: HashMap<usize, usize>,
}

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

impl Floor {
    fn new(compressed_xs: HashMap<usize, usize>, compressed_ys: HashMap<usize, usize>) -> Self {
        let width = compressed_xs.values().max().unwrap() + 1;
        let height = compressed_ys.values().max().unwrap() + 1;

        Self {
            tiles: vec![vec![Tile::Inside; width]; height],
            compressed_xs,
            compressed_ys,
        }
    }

    fn add_tiles(&mut self, positions: &[Position]) {
        let pairs: Vec<_> = positions
            .iter()
            .circular_tuple_windows()
            .map(|(pos1, pos2)| (self.compressed(pos1), self.compressed(pos2)))
            .collect();

        for (pos1, pos2) in pairs {
            assert!(pos1.left == pos2.left || pos1.top == pos2.top);

            if pos1.left == pos2.left {
                // vertical
                let y_min = cmp::min(pos1.top, pos2.top);
                let y_max = cmp::max(pos1.top, pos2.top);

                for y in y_min..=y_max {
                    self.tiles[y][pos1.left] = Tile::Boundary
                }
            } else {
                // horizontal
                let x_min = cmp::min(pos1.left, pos2.left);
                let x_max = cmp::max(pos1.left, pos2.left);

                for x in x_min..=x_max {
                    self.tiles[pos1.top][x] = Tile::Boundary;
                }
            }
        }
    }

    fn compressed(&self, position: &Position) -> Position {
        Position {
            left: *self.compressed_xs.get(&position.left).unwrap(),
            top: *self.compressed_ys.get(&position.top).unwrap(),
        }
    }

    fn fill(&mut self) {
        let mut tiles = vec![Position { top: 0, left: 0 }];

        while let Some(pos) = tiles.pop() {
            self.tiles[pos.top][pos.left] = Tile::Outside;

            for (row_offset, col_offset) in DIRECTIONS {
                let new_row = pos.top as isize + row_offset;
                if new_row < 0 {
                    continue;
                }

                let new_col = pos.left as isize + col_offset;
                if new_col < 0 {
                    continue;
                }

                if let Some(row) = self.tiles.get(new_row as usize)
                    && matches!(row.get(new_col as usize), Some(Tile::Inside))
                {
                    tiles.push(Position {
                        top: new_row as usize,
                        left: new_col as usize,
                    });
                }
            }
        }
    }

    fn max_area_within_polygon(&self, positions: &[Position]) -> usize {
        positions
            .iter()
            .enumerate()
            .flat_map(|(i, pos1)| positions.iter().skip(i + 1).map(|pos2| (*pos1, *pos2)))
            .filter(|(pos1, pos2)| {
                let pos1_compressed = self.compressed(pos1);
                let pos2_compressed = self.compressed(pos2);

                let x_min = cmp::min(pos1_compressed.left, pos2_compressed.left);
                let x_max = cmp::max(pos1_compressed.left, pos2_compressed.left);

                let y_min = cmp::min(pos1_compressed.top, pos2_compressed.top);
                let y_max = cmp::max(pos1_compressed.top, pos2_compressed.top);

                (x_min..=x_max)
                    .flat_map(|x| (y_min..=y_max).map(move |y| (x, y)))
                    .all(|(x, y)| matches!(self.tiles[y][x], Tile::Inside | Tile::Boundary))
            })
            .map(|(pos1, pos2)| area(&pos1, &pos2))
            .max()
            .unwrap()
    }
}

fn part_two(input: &str) -> usize {
    let positions: Vec<Position> = input.lines().map(Position::from).collect();

    let mut xs: Vec<usize> = positions.clone().iter().map(|pos| pos.left).collect();
    let mut ys: Vec<usize> = positions.clone().iter().map(|pos| pos.top).collect();

    xs.push(usize::MIN);
    xs.push(usize::MAX);
    xs.sort_unstable();
    xs.dedup();
    let compressed_xs = HashMap::from_iter(xs.iter().enumerate().map(|(i, x)| (*x, i)));

    ys.push(usize::MIN);
    ys.push(usize::MAX);
    ys.sort_unstable();
    ys.dedup();
    let compressed_ys = HashMap::from_iter(ys.iter().enumerate().map(|(i, y)| (*y, i)));

    let mut floor = Floor::new(compressed_xs, compressed_ys);
    floor.add_tiles(&positions);
    floor.fill();
    floor.max_area_within_polygon(&positions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_nine_part_one() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(part_one(input), 50);
    }

    #[test]
    fn day_nine_part_two() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(part_two(input), 24);
    }
}
