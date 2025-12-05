use std::{cmp, ops, time::Instant};

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

fn part_one(input: &str) -> usize {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();

    let ranges = ranges
        .lines()
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            let start: usize = start.parse().unwrap();
            let end: usize = end.parse().unwrap();

            start..=end
        })
        .collect::<Vec<_>>();

    ingredients
        .lines()
        .filter(|id| {
            let id: usize = id.parse().unwrap();
            ranges.iter().any(|range| range.contains(&id))
        })
        .count()
}

#[derive(Debug)]
struct Ranges {
    inner: Vec<ops::RangeInclusive<usize>>,
}

impl Ranges {
    fn new() -> Self {
        Self { inner: Vec::new() }
    }

    fn union(&mut self, other_range: ops::RangeInclusive<usize>) {
        let intersection = self
            .inner
            .iter()
            .position(|range| has_intersection(range, &other_range));

        if let Some(intersection_index) = intersection {
            let removed = self.inner.remove(intersection_index);
            let new_range = merge_ranges(removed, other_range);
            self.union(new_range);
        } else {
            let insert_index = self
                .inner
                .binary_search_by(|range| range.start().cmp(other_range.start()))
                .unwrap_or(0);

            self.inner.insert(insert_index, other_range);
        }
    }

    fn count(&self) -> usize {
        self.inner.iter().map(|range| range.clone().count()).sum()
    }
}

fn merge_ranges(
    removed: ops::RangeInclusive<usize>,
    other_range: ops::RangeInclusive<usize>,
) -> ops::RangeInclusive<usize> {
    let start = cmp::min(removed.start(), other_range.start());
    let end = cmp::max(removed.end(), other_range.end());
    *start..=*end
}

fn has_intersection(
    range: &ops::RangeInclusive<usize>,
    other_range: &ops::RangeInclusive<usize>,
) -> bool {
    let start = cmp::max(range.start(), other_range.start());
    let end = cmp::min(range.end(), other_range.end());

    let intersection = start..=end;

    !intersection.is_empty()
}

fn part_two(input: &str) -> usize {
    let (ranges, _) = input.split_once("\n\n").unwrap();

    ranges
        .lines()
        .map(|range| {
            let (start, end) = range.split_once('-').unwrap();
            let start: usize = start.parse().unwrap();
            let end: usize = end.parse().unwrap();

            start..=end
        })
        .fold(Ranges::new(), |mut set, range| {
            set.union(range);
            set
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_five_part_one() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        assert_eq!(part_one(input), 3);
    }

    #[test]
    fn day_five_part_two() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

        assert_eq!(part_two(input), 14);
    }
}
