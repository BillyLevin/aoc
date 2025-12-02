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

fn part_one(input: &str) -> u64 {
    let mut result = 0;
    let mut dial: i32 = 50;

    for line in input.lines() {
        let (direction, count) = line.split_at(1);
        let count: i32 = count.parse().expect("count should be a number");

        match direction {
            "R" => dial += count,
            "L" => dial -= count,
            _ => unreachable!("direction should be `R` or `L`"),
        }

        dial %= 100;

        if dial == 0 {
            result += 1;
        }
    }

    result
}

fn part_two(input: &str) -> u64 {
    let mut result = 0;
    let mut dial: i32 = 50;

    for line in input.lines() {
        let (direction, count) = line.split_at(1);
        let count: i32 = count.parse().expect("count should be a number");

        match direction {
            "R" => {
                for _ in 0..count {
                    dial += 1;
                    dial %= 100;
                    if dial == 0 {
                        result += 1;
                    }
                }
            }
            "L" => {
                for _ in 0..count {
                    dial -= 1;
                    dial %= 100;
                    if dial == 0 {
                        result += 1;
                    }
                }
            }
            _ => unreachable!("direction should be `R` or `L`"),
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_one_part_one() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(part_one(input), 3);
    }

    #[test]
    fn day_one_part_two() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

        assert_eq!(part_two(input), 6);
    }
}
