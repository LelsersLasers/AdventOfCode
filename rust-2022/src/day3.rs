use crate::utils;

pub fn day3() {
    let day3_part1_solution = part1();
    println!("Day 3 - Part 1: {day3_part1_solution}");

    // let day3_part2_solution = part2();
    // println!("Day 3 - Part 2: {day3_part2_solution}");
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn part1() -> u32 {
    let input_text = utils::read_file("inputs/day3.txt");
    input_text
        .split('\n') // split at line breaks
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2)) // cut in half
        .map(|(compartment1, compartment2)| {
            // find duplicate letter
            compartment1.chars().find_map(|c| {
                if compartment2.contains(c) {
                    Some(c)
                } else {
                    None
                }
            })
        })
        .map(|letter| (ALPHABET.find(|l| l == letter.unwrap()).unwrap() + 1) as u32)
        .sum()
}

pub fn part2() -> u32 {
    let input_text = utils::read_file("inputs/day3.txt");
    todo!()
}
