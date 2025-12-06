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
    let lines: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let input_count = lines.len();
    let problem_count = lines[0].len();

    let mut result = 0;

    for i in 0..problem_count {
        let values = &lines[..input_count - 1]
            .iter()
            .map(|value| value[i].parse::<u64>().unwrap());

        let op = lines[input_count - 1][i];

        match op {
            "+" => result += values.clone().sum::<u64>(),
            "*" => result += values.clone().product::<u64>(),
            _ => panic!("invalid input"),
        };
    }

    result
}

#[derive(Debug, Clone, Copy)]
enum Operation {
    Add,
    Multiply,
}

fn part_two(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let line_length = lines[0].len();
    let line_count = lines.len();

    let mut result = 0;
    let mut operation = Operation::Add;

    let mut values: Vec<u64> = Vec::new();

    for col in (0..line_length).rev() {
        let mut col_values: Vec<u64> = Vec::new();

        for row in lines.iter().take(line_count) {
            let value = &row[col..=col];
            if value.trim().is_empty() {
                continue;
            }

            match value {
                "+" => operation = Operation::Add,
                "*" => operation = Operation::Multiply,
                _ => {
                    col_values.push(value.parse().unwrap());
                }
            }
        }

        if col_values.is_empty() {
            result += match operation {
                Operation::Add => values.clone().into_iter().sum::<u64>(),
                Operation::Multiply => values.clone().into_iter().product::<u64>(),
            };
            values.clear();
        } else {
            let value = col_values
                .iter()
                .rev()
                .enumerate()
                .map(|(i, part)| part * (10u64.pow(i as u32)))
                .sum::<u64>();

            values.push(value);
        }
    }

    result += match operation {
        Operation::Add => values.clone().into_iter().sum::<u64>(),
        Operation::Multiply => values.clone().into_iter().product::<u64>(),
    };

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_six_part_one() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

        assert_eq!(part_one(input), 4277556);
    }

    #[test]
    fn day_six_part_two() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

        assert_eq!(part_two(input), 3263827);
    }
}
