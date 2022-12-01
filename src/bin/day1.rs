fn main() {
    let input = include_str!("day1.txt");
    println!("PART 1: {}", solve_part1(input));
    println!("PART 2: {}", solve_part2(input));
}

#[test]
fn test() {
    let input = include_str!("day1_test.txt");
    assert_eq!(solve_part1(input), 24000);
    assert_eq!(solve_part2(input), 45000);
}

fn solve_part1(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|cals| cals.parse::<i32>().unwrap()).sum())
        .max()
        .unwrap_or(0)
}

fn solve_part2(input: &str) -> i32 {
    let mut totals: Vec<i32> = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|cals| cals.parse::<i32>().unwrap()).sum())
        .collect();
    totals.sort();
    totals.iter().rev().take(3).sum()
}
