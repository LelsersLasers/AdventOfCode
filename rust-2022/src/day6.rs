use crate::utils;

pub fn day6() {
    let day6_part1_solution = part1();
    println!("Day 6 - Part 1: {day6_part1_solution}");

    // let day6_part2_solution = part2();
    // println!("day 6 - part 2: {day6_part2_solution}");
}

pub fn part1() -> usize {
    let input_text = utils::read_file("inputs/day6.txt");
    let first_line: &str = input_text.lines().take(1).collect::<Vec<&str>>()[0];
    let letters = first_line.chars().collect::<Vec<char>>();

    for i in 0..letters.len() - 4 {
        let mut ls = Vec::new();
        let mut all_dif = true;
        for j in 0..4 {
            if ls.contains(&letters[i + j]) {
                all_dif = false;
            }
            ls.push(letters[i + j]);
        }
        if all_dif {
            return i + 4;
        }
    }

    panic!("Start marker not found!")
}

pub fn part2() -> usize {
    let input_text = utils::read_file("inputs/day6.txt");
    todo!()
}
