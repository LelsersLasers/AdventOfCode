use crate::utils;

pub fn day5() {
    let day5_part1_solution = part1();
    println!("Day 5 - Part 1: {day5_part1_solution}");

    // let day5_part2_solution = part2();
    // println!("Day 5 - Part 2: {day5_part2_solution}");
}

struct Crate {
    letter: char,
}
impl Crate {
    fn new(letter: char) -> Self {
        Self { letter }
    }
}

struct Stack {
    crates: Vec<Crate>, // crates[0] = bottom crate
}
impl Stack {
    fn new() -> Self {
        Self { crates: Vec::new() }
    }
}

pub fn part1() -> String {
    let input_text = utils::read_file("inputs/day5.txt");

    let crate_lines_str = input_text.split("\n\n").next().unwrap();
    let crate_lines = crate_lines_str.split('\n').rev().collect::<Vec<&str>>();
    let stack_count = crate_lines[0]
        .split_whitespace()
        .map(|c| c.parse::<u32>().unwrap())
        .max()
        .unwrap();

    let mut stacks = Vec::new();
    for _i in 0..stack_count {
        stacks.push(Stack::new());
    }

    for i in 1..crate_lines.len() {
        // 1 => skip line with numbers
        let line = crate_lines[i];
        for j in 0..stack_count {
            let letter_idx = 1 + 4 * j;
            let letter = line.chars().nth(letter_idx as usize).unwrap();
            if letter != ' ' {
                stacks[j as usize].crates.push(Crate::new(letter));
            }
        }
    }

    let move_lines_str = input_text.split("\n\n").nth(1).unwrap();
    let move_lines = move_lines_str.split('\n').collect::<Vec<&str>>();
    for move_line in move_lines {
        let mut line_parts = move_line.split_whitespace();
        let move_count = line_parts.nth(1).unwrap().parse::<u32>().unwrap();
        let start_stack = line_parts.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let end_stack = line_parts.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        // stacks[start_stack].move_crates(move_count, &mut stacks[end_stack]);
        for _i in 0..move_count {
            let crate_to_move = stacks[start_stack].crates.pop().unwrap();
            stacks[end_stack].crates.push(crate_to_move);
        }
    }

    let mut out_string = "".to_string();
    for stack in stacks {
        let top_crate = stack.crates.last().unwrap().letter;
        out_string += &top_crate.to_string();
    }

    out_string
}

pub fn part2() -> u32 {
    let input_text = utils::read_file("inputs/day5.txt");
    todo!()
}
