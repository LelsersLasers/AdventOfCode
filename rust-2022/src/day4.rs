use crate::utils;

#[allow(dead_code)]
pub fn day4() {
    let day4_part1_solution = part1();
    println!("Day 4 - Part 1: {day4_part1_solution}");

    let day4_part2_solution = part2();
    println!("Day 4 - Part 2: {day4_part2_solution}");
}

pub fn part1() -> u32 {
    let input_text = utils::read_file("inputs/day4.txt");
    input_text
        .split('\n')
        .map(|line| line.split(',').collect::<Vec<&str>>())
        .map(|range_strs| {
            let r1 = range_strs[0];
            let r2 = range_strs[1];

            let mut r1s = r1.split('-');
            let r1a = r1s.next().unwrap().parse::<u32>().unwrap();
            let r1b = r1s.next().unwrap().parse::<u32>().unwrap();

            let mut r2s = r2.split('-');
            let r2a = r2s.next().unwrap().parse::<u32>().unwrap();
            let r2b = r2s.next().unwrap().parse::<u32>().unwrap();

            ((r1a, r1b), (r2a, r2b))
        })
        .filter(|((r1a, r1b), (r2a, r2b))| (r1a <= r2a && r1b >= r2b) || (r2a <= r1a && r2b >= r1b))
        .count() as u32
}

pub fn part2() -> u32 {
    let input_text = utils::read_file("inputs/day4.txt");
    input_text
        .split('\n')
        .map(|line| line.split(',').collect::<Vec<&str>>())
        .map(|range_strs| {
            let r1 = range_strs[0];
            let r2 = range_strs[1];

            let mut r1s = r1.split('-');
            let r1a = r1s.next().unwrap().parse::<u32>().unwrap();
            let r1b = r1s.next().unwrap().parse::<u32>().unwrap();

            let mut r2s = r2.split('-');
            let r2a = r2s.next().unwrap().parse::<u32>().unwrap();
            let r2b = r2s.next().unwrap().parse::<u32>().unwrap();

            ((r1a, r1b), (r2a, r2b))
        })
        .filter(|((r1a, r1b), (r2a, r2b))| {
            let mut r1 = r1a.to_owned()..=r1b.to_owned();
            let r2 = r2a.to_owned()..=r2b.to_owned();
            r1.any(|x| r2.contains(&x))
        })
        .count() as u32
}
