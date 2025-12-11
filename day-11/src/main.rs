use std::{collections::HashMap, time::Instant};

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

fn part_one(input: &str) -> usize {
    let map: HashMap<&str, Vec<&str>> = HashMap::from_iter(input.lines().map(|line| {
        let (device, outputs) = line.split_once(':').unwrap();
        (device, outputs.split_whitespace().collect())
    }));

    fn traverse(device: &str, map: &HashMap<&str, Vec<&str>>) -> usize {
        map.get(device)
            .unwrap()
            .iter()
            .map(|&output| match output {
                "out" => 1,
                _ => traverse(output, map),
            })
            .sum()
    }

    traverse("you", &map)
}

fn part_two(input: &str) -> usize {
    let map: HashMap<&str, Vec<&str>> = HashMap::from_iter(input.lines().map(|line| {
        let (device, outputs) = line.split_once(':').unwrap();
        (device, outputs.split_whitespace().collect())
    }));

    let mut cache = HashMap::new();

    fn traverse<'src>(
        device: &'src str,
        visited_fft: bool,
        visited_dac: bool,
        map: &'src HashMap<&'src str, Vec<&'src str>>,
        cache: &mut HashMap<(&'src str, bool, bool), usize>,
    ) -> usize {
        if let Some(result) = cache.get(&(device, visited_fft, visited_dac)) {
            return *result;
        }

        let result = map
            .get(device)
            .unwrap()
            .iter()
            .map(|&output| match output {
                "out" => {
                    if visited_fft && visited_dac {
                        1
                    } else {
                        0
                    }
                }
                "fft" => traverse(output, true, visited_dac, map, cache),
                "dac" => traverse(output, visited_fft, true, map, cache),
                _ => traverse(output, visited_fft, visited_dac, map, cache),
            })
            .sum();

        cache.insert((device, visited_fft, visited_dac), result);
        result
    }

    traverse("svr", false, false, &map, &mut cache)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_eleven_part_one() {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        assert_eq!(part_one(input), 5);
    }

    #[test]
    fn day_eleven_part_two() {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        assert_eq!(part_two(input), 2);
    }
}
