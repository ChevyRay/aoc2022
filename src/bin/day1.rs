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
        .map(|items| {
            items
                .lines()
                .map(|calories| calories.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max()
        .unwrap_or(0)
}

fn solve_part2(input: &str) -> i32 {
    let mut max_for_each: Vec<i32> = input
        .split("\n\n")
        .map(|items| {
            items
                .lines()
                .map(|item| item.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect();
    max_for_each.sort();
    max_for_each.iter().rev().take(3).sum::<i32>()
}
