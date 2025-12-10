use std::{iter::Peekable, str::Chars, time::Instant};

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");

    let start = Instant::now();
    let answer = part_one(input);
    let elapsed = start.elapsed();
    println!("Part one: {answer}, took {elapsed:?}");
}

#[derive(Debug)]
struct MachineParser<'src> {
    chars: Peekable<Chars<'src>>,
}

impl<'src> MachineParser<'src> {
    fn new(source: &'src str) -> Self {
        Self {
            chars: source.chars().peekable(),
        }
    }

    fn parse(&mut self) -> Machine {
        let lights = self.parse_lights();
        self.eat_whitespace();
        let buttons = self.parse_buttons();

        Machine { lights, buttons }
    }

    fn parse_lights(&mut self) -> usize {
        self.expect('[');

        let mut lights = 0;
        let mut shift = 0usize;

        loop {
            let ch = self.chars.next().unwrap();

            match ch {
                '.' => {}
                '#' => lights |= 1 << shift,
                ']' => break,
                _ => unreachable!(),
            };

            shift += 1;
        }

        lights
    }

    fn parse_buttons(&mut self) -> Vec<usize> {
        let mut buttons = Vec::new();

        while let Some(ch) = self.chars.peek()
            && *ch == '('
        {
            let mut lights = 0usize;

            self.chars.next();

            let mut num_str = String::new();

            loop {
                match self.chars.next().unwrap() {
                    next @ '0'..='9' => num_str.push(next),
                    ',' => {
                        let num: usize = num_str.parse().unwrap();
                        lights += 2usize.pow(num as u32);
                        num_str.clear();
                    }
                    ')' => {
                        let num: usize = num_str.parse().unwrap();
                        lights += 2usize.pow(num as u32);
                        buttons.push(lights);
                        break;
                    }
                    _ => unreachable!(),
                }
            }

            self.eat_whitespace();
        }

        buttons
    }

    fn expect(&mut self, ch: char) {
        let next = self.chars.next();
        assert!(next == Some(ch));
    }

    fn eat_whitespace(&mut self) {
        while let Some(ch) = self.chars.peek() {
            if !ch.is_whitespace() {
                break;
            }

            self.chars.next();
        }
    }
}

#[derive(Debug)]
struct Machine {
    lights: usize,
    buttons: Vec<usize>,
}

fn part_one(input: &str) -> usize {
    input
        .lines()
        .map(|line| MachineParser::new(line).parse())
        .map(|machine| {
            (1..=machine.buttons.len())
                .find(|&presses| {
                    machine.buttons.iter().combinations(presses).any(|buttons| {
                        buttons
                            .into_iter()
                            .fold(0usize, |lights, button| lights ^ button)
                            == machine.lights
                    })
                })
                .unwrap()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_ten_part_one() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(part_one(input), 7);
    }
}
