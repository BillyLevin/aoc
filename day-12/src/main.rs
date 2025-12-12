use std::time::Instant;

fn main() {
    let input = include_str!("../input.txt");

    let start = Instant::now();
    let answer = part_one(input);
    let elapsed = start.elapsed();
    println!("Part one: {answer}, took {elapsed:?}");
}

#[derive(Debug)]
struct Shape {
    units: Vec<Vec<Unit>>,
}

impl Shape {
    fn new(shape: &str) -> Self {
        let units = shape.lines().skip(1).fold(Vec::new(), |mut units, line| {
            units.push(
                line.chars()
                    .map(|ch| match ch {
                        '#' => Unit::Occupied,
                        '.' => Unit::Empty,
                        _ => unreachable!(),
                    })
                    .collect(),
            );
            units
        });

        Self { units }
    }

    fn area(&self) -> usize {
        self.units
            .iter()
            .flat_map(|row| {
                row.iter().map(|unit| match unit {
                    Unit::Occupied => 1,
                    Unit::Empty => 0,
                })
            })
            .sum()
    }
}

#[derive(Debug)]
enum Unit {
    Occupied,
    Empty,
}

fn part_one(input: &str) -> usize {
    let input = input.split("\n\n").collect::<Vec<_>>();
    let (regions, shapes) = input.split_last().unwrap();

    let shapes: Vec<Shape> = shapes.iter().map(|shape| Shape::new(shape)).collect();

    regions
        .lines()
        .filter(|region| {
            let (dimensions, counts) = region.split_once(':').unwrap();
            let counts = counts
                .split_whitespace()
                .map(|count| count.parse::<usize>().unwrap())
                .enumerate();

            let (width, height) = dimensions.split_once('x').unwrap();
            let width: usize = width.parse().unwrap();
            let height: usize = height.parse().unwrap();
            let area = width * height;

            counts
                .map(|(shape_index, count)| shapes[shape_index].area() * count)
                .sum::<usize>()
                <= area
        })
        .count()
}
