use std::str::FromStr;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part one: {}", part_one(input));

    let input = include_str!("../input.txt");
    println!("Part two: {}", part_two(input));
}

struct Range {
    start: u64,
    end: u64,
}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').unwrap();

        Ok(Range {
            start: start.parse().unwrap(),
            end: end.parse().unwrap(),
        })
    }
}

fn part_one(input: &str) -> u64 {
    let mut result = 0;

    for range in input.split(',') {
        let range: Range = range.parse().unwrap();

        for id in range.start..=range.end {
            let id_string = id.to_string();

            if id_string.len() % 2 == 0 {
                let (left, right) = id_string.split_at(id_string.len() / 2);

                if left == right {
                    result += id;
                }
            }
        }
    }

    result
}

fn part_two(input: &str) -> u64 {
    let mut result = 0;

    for range in input.split(',') {
        let range: Range = range.parse().unwrap();

        for id in range.start..=range.end {
            let id_string = id.to_string();

            for i in 1..=(id_string.len() / 2) {
                let candidate = &id_string[0..i];

                if id_string.len() % i == 0 && candidate.repeat(id_string.len() / i) == id_string {
                    result += id;
                    break;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_two_part_one() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        assert_eq!(part_one(input), 1227775554);
    }

    #[test]
    fn day_two_part_two() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        assert_eq!(part_two(input), 4174379265);
    }
}
