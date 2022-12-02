fn main() {
    let input = include_str!("day2.txt");
    println!("PART 1: {}", solve_part1(input));
    println!("PART 2: {}", solve_part2(input));
    println!("PART 1 (alt): {}", solve_part1_alt(input));
    println!("PART 2 (alt): {}", solve_part2_alt(input));
}

#[test]
fn test() {
    let input = include_str!("day2_test.txt");
    assert_eq!(solve_part1(input), 15);
    assert_eq!(solve_part2(input), 12);
    assert_eq!(solve_part1_alt(input), 15);
    assert_eq!(solve_part2_alt(input), 12);
}

fn solve_part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| match line.split_once(' ').unwrap() {
            ("A", "X") => 4,
            ("B", "X") => 1,
            ("C", "X") => 7,
            ("A", "Y") => 8,
            ("B", "Y") => 5,
            ("C", "Y") => 2,
            ("A", "Z") => 3,
            ("B", "Z") => 9,
            ("C", "Z") => 6,
            _ => panic!(),
        })
        .sum()
}

fn solve_part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| match line.split_once(' ').unwrap() {
            ("A", "X") => 3,
            ("B", "X") => 1,
            ("C", "X") => 2,
            ("A", "Y") => 4,
            ("B", "Y") => 5,
            ("C", "Y") => 6,
            ("A", "Z") => 8,
            ("B", "Z") => 9,
            ("C", "Z") => 7,
            _ => panic!(),
        })
        .sum()
}

fn solve_part1_alt(input: &str) -> i32 {
    input
        .lines()
        .map(str::as_bytes)
        .map(|line| (((line[0] - b'A') << 2) | (line[2] - b'X')) as usize)
        .map(|i| [4, 8, 3, 0, 1, 5, 9, 0, 7, 2, 6][i])
        .sum()
}

fn solve_part2_alt(input: &str) -> i32 {
    input
        .lines()
        .map(str::as_bytes)
        .map(|line| (((line[0] - b'A') << 2) | (line[2] - b'X')) as usize)
        .map(|i| [3, 4, 8, 0, 1, 5, 9, 0, 2, 6, 7][i])
        .sum()
}
