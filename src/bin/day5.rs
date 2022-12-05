fn main() {
    let input = include_str!("day5.txt");
    println!("PART 1: {}", solve_part1(input, 9));
    println!("PART 2: {}", solve_part2(input, 9));
}

#[test]
fn test() {
    let input = include_str!("day5_test.txt");
    assert_eq!(solve_part1(input, 3), "CMZ");
    assert_eq!(solve_part2(input, 3), "MCD");
}

fn create_stacks(txt: &str, count: usize) -> Vec<Vec<u8>> {
    let mut stacks = Vec::new();
    stacks.resize_with(count, Vec::new);
    for line in txt.lines().rev().skip(1).map(str::as_bytes) {
        for (i, p) in ((0..line.len()).step_by(4)).enumerate() {
            if line[p] == b'[' {
                stacks[i].push(line[p + 1]);
            }
        }
    }
    stacks
}

fn perform_op(stacks: &mut Vec<Vec<u8>>, op: &str, keep_order: bool) {
    let nums: Vec<usize> = op
        .split(' ')
        .filter_map(|txt| txt.parse::<usize>().ok())
        .collect();
    for _ in 0..nums[0] {
        let c = stacks[nums[1] - 1].pop().unwrap();
        stacks[nums[2] - 1].push(c);
    }
    if keep_order {
        let len = stacks[nums[2] - 1].len();
        stacks[nums[2] - 1][(len - nums[0])..].reverse();
    }
}

fn get_tops(stacks: Vec<Vec<u8>>) -> String {
    stacks
        .into_iter()
        .map(|s| char::from_u32(*s.last().unwrap() as u32).unwrap())
        .collect()
}

fn solve_part1(input: &str, num_stacks: usize) -> String {
    let (stacks, ops) = input.split_once("\n\n").unwrap();
    let mut stacks = create_stacks(stacks, num_stacks);
    for op in ops.lines() {
        perform_op(&mut stacks, op, false);
    }
    get_tops(stacks)
}

fn solve_part2(input: &str, num_stacks: usize) -> String {
    let (stacks, ops) = input.split_once("\n\n").unwrap();
    let mut stacks = create_stacks(stacks, num_stacks);
    for op in ops.lines() {
        perform_op(&mut stacks, op, true);
    }
    get_tops(stacks)
}
