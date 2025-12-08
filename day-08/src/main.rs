use std::{cmp, time::Instant};

fn main() {
    let input = include_str!("../input.txt");

    let start = Instant::now();
    let answer = part_one(input, 1000);
    let elapsed = start.elapsed();
    println!("Part one: {answer}, took {elapsed:?}");

    let start = Instant::now();
    let answer = part_two(input);
    let elapsed = start.elapsed();
    println!("Part two: {answer}, took {elapsed:?}");
}

#[derive(Debug, Clone, Copy)]
struct Junction {
    index: usize,
    position: Position,
}

impl From<(usize, &str)> for Junction {
    fn from((index, position): (usize, &str)) -> Self {
        Self {
            index,
            position: Position::from(position),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: u64,
    y: u64,
    z: u64,
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        let mut values = value.splitn(3, ',');

        Self {
            x: values.next().unwrap().parse().unwrap(),
            y: values.next().unwrap().parse().unwrap(),
            z: values.next().unwrap().parse().unwrap(),
        }
    }
}

impl Position {
    fn distance_squared(&self, other: &Position) -> u64 {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

#[derive(Debug)]
struct Distance {
    value: u64,

    junction1: usize,
    junction2: usize,
}

impl Distance {
    fn new(junction1: &Junction, junction2: &Junction) -> Self {
        Self {
            value: junction1.position.distance_squared(&junction2.position),
            junction1: junction1.index,
            junction2: junction2.index,
        }
    }
}

impl PartialEq for Distance {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Eq for Distance {}

impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

/// NOTE: Disjoint Set union find
#[derive(Debug)]
struct Circuits {
    parents: Vec<usize>,
    lengths: Vec<usize>,
}

impl Circuits {
    fn new(junctions: &[Junction]) -> Self {
        Self {
            parents: junctions.iter().map(|junction| junction.index).collect(),
            lengths: vec![1; junctions.len()],
        }
    }

    fn connect(&mut self, junction1: usize, junction2: usize) {
        let root1 = self.find(junction1);
        let root2 = self.find(junction2);

        if root1 != root2 {
            self.parents[root2] = root1;
            self.lengths[root1] += self.lengths[root2];
        }
    }

    fn find(&self, junction: usize) -> usize {
        let parent = self.parents[junction];

        if parent == junction {
            junction
        } else {
            self.find(parent)
        }
    }

    fn all_connected(&self) -> bool {
        let root = self.find(0);
        self.lengths[root] == self.parents.len()
    }
}

fn part_one(input: &str, connections: usize) -> usize {
    let junctions: Vec<_> = input.lines().enumerate().map(Junction::from).collect();

    let mut distances =
        junctions
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut distances, (i, junction)| {
                distances.extend(
                    junctions[i + 1..]
                        .iter()
                        .map(|junction2| Distance::new(junction, junction2)),
                );
                distances
            });

    distances.sort_unstable();

    let mut circuits = Circuits::new(&junctions);

    for distance in distances.iter().take(connections) {
        circuits.connect(distance.junction1, distance.junction2);
    }

    let mut lengths: Vec<_> = (0..circuits.parents.len())
        .filter(|&i| i == circuits.find(i))
        .map(|i| circuits.lengths[i])
        .collect();

    lengths.sort_by(|a, b| b.cmp(a));

    lengths.iter().take(3).product()
}

fn part_two(input: &str) -> u64 {
    let junctions: Vec<_> = input.lines().enumerate().map(Junction::from).collect();

    let mut distances =
        junctions
            .iter()
            .enumerate()
            .fold(Vec::new(), |mut distances, (i, junction)| {
                distances.extend(
                    junctions[i + 1..]
                        .iter()
                        .map(|junction2| Distance::new(junction, junction2)),
                );
                distances
            });

    distances.sort_unstable();

    let mut circuits = Circuits::new(&junctions);

    for distance in distances {
        circuits.connect(distance.junction1, distance.junction2);

        if circuits.all_connected() {
            return junctions[distance.junction1].position.x
                * junctions[distance.junction2].position.x;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_eight_part_one() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(part_one(input, 10), 40);
    }

    #[test]
    fn day_eight_part_two() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        assert_eq!(part_two(input), 25272);
    }
}
