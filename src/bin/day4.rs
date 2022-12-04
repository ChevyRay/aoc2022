use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("day4.txt");
    println!("PART 1: {}", solve_part1(input));
    println!("PART 2: {}", solve_part2(input));
}

#[test]
fn test() {
    let input = include_str!("day4_test.txt");
    assert_eq!(solve_part1(input), 2);
    assert_eq!(solve_part2(input), 4);
}

fn range(txt: &str) -> RangeInclusive<i32> {
    txt.split_once('-')
        .map(|(a, b)| a.parse().unwrap()..=b.parse().unwrap())
        .unwrap()
}

fn solve_part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(a, b)| (range(a), range(b)))
        .filter(|(a, b)| {
            (a.contains(b.start()) && a.contains(b.end()))
                || (b.contains(a.start()) && b.contains(a.end()))
        })
        .count()
}

fn solve_part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(a, b)| (range(a), range(b)))
        .filter(|(a, b)| {
            a.contains(b.start())
                || a.contains(b.end())
                || b.contains(a.start())
                || b.contains(a.end())
        })
        .count()
}
