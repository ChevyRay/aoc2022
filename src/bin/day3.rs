fn main() {
    let input = include_str!("day3.txt");
    println!("PART 1: {}", solve_part1(input));
    println!("PART 2: {}", solve_part2(input));
}

#[test]
fn test() {
    let input = include_str!("day3_test.txt");
    assert_eq!(solve_part1(input), 157);
    assert_eq!(solve_part2(input), 70);
}

fn priority(chr: &u8) -> i32 {
    if chr >= &b'a' {
        1 + (chr - b'a') as i32
    } else {
        27 + (chr - b'A') as i32
    }
}

fn solve_part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.as_bytes().split_at(line.len() / 2))
        .filter_map(|(a, b)| a.iter().find(|chr| b.contains(chr)))
        .map(priority)
        .sum()
}

fn solve_part2(input: &str) -> i32 {
    input
        .lines()
        .map(str::as_bytes)
        .collect::<Vec<_>>()
        .chunks_exact(3)
        .filter_map(|sacks| {
            sacks[0]
                .iter()
                .find(|chr| sacks[1].contains(chr) && sacks[2].contains(chr))
        })
        .map(priority)
        .sum()
}
