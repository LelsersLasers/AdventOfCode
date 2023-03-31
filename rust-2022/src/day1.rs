use crate::utils;

#[allow(dead_code)]
pub fn day1() {
    let day1_part1_solution = part1();
    println!("Day 1 - Part 1: {day1_part1_solution}");

    let day1_part2_solution = part2();
    println!("Day 1 - Part 2: {day1_part2_solution}");
}

pub fn part1() -> u32 {
    let input_text = utils::read_file("inputs/day1.txt");
    input_text
        .split("\n\n") // 2 line breaks inbetween elves
        .map(|elf| {
            elf.split("\n")
                .map(|calorie| calorie.parse::<u32>().unwrap())
                .sum()
        })
        .max()
        .unwrap()
}

pub fn part2() -> u32 {
    let input_text = utils::read_file("inputs/day1.txt");
    let mut elf_calories = input_text
        .split("\n\n") // 2 line breaks inbetween elves
        .map(|elf| {
            elf.split("\n")
                .map(|calorie| calorie.parse::<u32>().unwrap())
                .sum()
        })
        .collect::<Vec<u32>>();
    elf_calories.sort();
    elf_calories.iter().rev().take(3).sum()
}
