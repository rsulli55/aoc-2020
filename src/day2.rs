use aoc_runner_derive::aoc;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::num::ParseIntError;

pub struct Password<'a> {
    // this could be changed to a tuple, I was thinking that part 2
    // may use multiple requirements
    requirements: HashMap<char, (usize, usize)>,
    password: &'a str,
}

impl<'a> Password<'a> {
    fn is_valid_p1(&self) -> bool {
        let check_counts = |(letter, counts): (&char, &(usize, usize))| {
            let count = self.password.chars().filter(|l| l == letter).count();
            count >= counts.0 && count <= counts.1
        };

        self.requirements.iter().map(check_counts).all(|x| x)
    }

    fn is_valid_p2(&self) -> bool {
        let check_indices = |(letter, counts): (&char, &(usize, usize))| {
            // check if letter occurs at index `counts.0 - 1` xor `counts.1 - 1`
            let mut pos1 = false;
            let mut pos2 = false;
            if let Some(l) = self.password.chars().nth(counts.0 - 1) {
                if l == *letter {
                    pos1 = true;
                }
            }

            if let Some(l) = self.password.chars().nth(counts.1 - 1) {
                if l == *letter {
                    pos2 = true;
                }
            }

            (pos1 && !pos2) || (!pos1 && pos2)
        };

        self.requirements.iter().map(check_indices).all(|x| x)
    }
}

impl<'a> TryFrom<&'a str> for Password<'a> {
    type Error = ParseIntError;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let pieces: Vec<_> = s.split(' ').collect();

        // min and max counts occur in first piece on either side of `-`
        let counts: Vec<&str> = pieces[0].split('-').collect();
        let counts = (counts[0].parse::<usize>()?, counts[1].parse::<usize>()?);

        // letter occurs in second piece after in first index
        let letter = pieces[1].chars().next().unwrap();

        let mut requirements = HashMap::new();
        requirements.insert(letter, counts);

        // password is third piece
        let password = pieces[2];

        Ok(Password {
            requirements,
            password,
        })
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|s| Password::try_from(s).unwrap().is_valid_p1())
        .filter(|&x| x)
        .count()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|s| Password::try_from(s).unwrap().is_valid_p2())
        .filter(|&x| x)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample1() {
        assert_eq!(part1("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc"), 2);
    }

    #[test]
    fn part2_sample1() {
        assert_eq!(part2("1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc"), 1);
    }
}
