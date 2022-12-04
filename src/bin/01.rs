fn sum(input: &str) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
    let mut elves: Vec<u32> = vec![];
    let mut sum: u32 = 0;

    for line in input.lines() {
        if line.is_empty() {
            elves.push(sum);
            sum = 0;
            continue;
        }

        let cals: u32 = line.trim().parse()?;
        sum += cals;
    }

    // trailing record if missing eol
    if sum > 0 {
        elves.push(sum);
    }
    Ok(elves)
}

pub fn part_one(input: &str) -> Option<u32> {
    let elves = sum(input).unwrap();
    Some(*elves.iter().max().unwrap() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elves = sum(input).unwrap();
    elves.sort();
    Some(elves[elves.len() - 3..elves.len()].iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("examples", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24_000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45_000));
    }
}
