use crate::utils;

#[allow(dead_code)]
pub fn day3() {
    let day3_part1_solution = part1();
    println!("Day 3 - Part 1: {day3_part1_solution}");

    let day3_part2_solution = part2();
    println!("Day 3 - Part 2: {day3_part2_solution}");
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn part1() -> u32 {
    let input_text = utils::read_file("inputs/day3.txt");
    input_text
        .split('\n') // split at line breaks
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2)) // cut in half
        .map(|(compartment1, compartment2)| {
            // find duplicate letter
            compartment1
                .chars()
                .find(|c| compartment2.contains(c.to_owned()))
        })
        .map(|letter| (ALPHABET.find(|l| l == letter.unwrap()).unwrap() + 1) as u32)
        .sum()
}

pub fn part2() -> u32 {
    let input_text = utils::read_file("inputs/day3.txt");

    let mut grouped_sacs = Vec::new();
    for (i, line) in input_text.lines().enumerate() {
        if i % 3 == 0 {
            grouped_sacs.push(Vec::new());
        }
        grouped_sacs.last_mut().unwrap().push(line);
    }

    let mut badge_letters = Vec::new();
    for grouped_sac in grouped_sacs {
        // unique between first 2
        let badge = grouped_sac[0]
            .chars()
            .find(|c| {
                grouped_sac[1].contains(c.to_owned()) && grouped_sac[2].contains(c.to_owned())
            })
            .unwrap();
        badge_letters.push(badge);
    }

    badge_letters
        .into_iter()
        .map(|letter| (ALPHABET.find(|l| l == letter).unwrap() + 1) as u32)
        .sum()
}
