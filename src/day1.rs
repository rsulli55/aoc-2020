use aoc_runner_derive::aoc;
use std::collections::HashSet;
use itertools::iproduct;

#[aoc(day1, part1)]
pub fn part1(input: &str) -> isize {
    let set: HashSet<isize> = input.lines().map(|s| s.parse().unwrap()).collect::<HashSet<_>>();
    // for each `n` in `set` look for `2020 - n`
    set.iter().filter(|&n| set.contains(&(2020 - *n))).map(|n| n * (2020 - n)).next().unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> isize {
    let set: HashSet<isize> = input.lines().map(|s| s.parse().unwrap()).collect::<HashSet<_>>();
    // for each `n`, `m` in `set` look for `2020 - (n + m)`
    iproduct!(&set, &set).filter(|&(n, m)| set.contains(&(2020 - (n + m)))).map(|(n, m)| n * m * (2020 - (n + m))).next().unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample1() {
        assert_eq!(part1("1721\n979\n366\n299\n675\n1456"), 514579);
    }

    #[test]
    fn part2_sample1() {
        assert_eq!(part2("1721\n979\n366\n299\n675\n1456"), 241861950);
    }
}
