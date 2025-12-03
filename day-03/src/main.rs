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

fn to_digit(ch: char) -> u64 {
    u64::from(ch.to_digit(10).unwrap())
}

fn part_one(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            // reversing the iter because `max_by_key()` returns the last element found if there are
            // multiple items that are equally maximum
            let (highest_rev_position, highest_value) = line
                .chars()
                .rev()
                .map(to_digit)
                .enumerate()
                .skip(1)
                .max_by_key(|(_i, num)| *num)
                .unwrap();

            let next_highest = line
                .chars()
                .rev()
                .map(to_digit)
                .take(highest_rev_position)
                .max()
                .unwrap();

            (highest_value * 10) + next_highest
        })
        .sum()
}

fn part_two(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            (1..=12)
                .rev()
                .fold(Vec::new(), |mut values, remaining| {
                    let take = values.last().map_or(usize::MAX, |(i, _num)| *i);
                    let (highest_rev_position, highest_value) = line
                        .chars()
                        .rev()
                        .map(to_digit)
                        .enumerate()
                        .take(take)
                        .skip(remaining - 1)
                        .max_by_key(|(_i, num)| *num)
                        .unwrap();

                    values.push((highest_rev_position, highest_value));
                    values
                })
                .into_iter()
                .map(|(_position, value)| value)
                .enumerate()
                .map(|(i, value)| value * 10u64.pow((12 - i - 1) as u32))
                .sum::<u64>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_three_part_one() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";

        assert_eq!(part_one(input), 357);
    }

    #[test]
    fn day_three_part_two() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";

        assert_eq!(part_two(input), 3121910778619);
    }
}
