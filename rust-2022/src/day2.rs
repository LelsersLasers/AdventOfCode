use crate::utils;

#[allow(dead_code)]
pub fn day2() {
    let day2_part1_solution = part1();
    println!("Day 2 - Part 1: {day2_part1_solution}");

    let day2_part2_solution = part2();
    println!("Day 2 - Part 2: {day2_part2_solution}");
}

fn score_round_part1(round: &str) -> u32 {
    let round_parts = round.split(' ').collect::<Vec<&str>>();
    let their_move = round_parts[0];
    let our_move = round_parts[1];

    // rock: A, X; paper: B, Y; scissors: C, Z
    let winner_moves = [["A", "X"], ["B", "Y"], ["C", "Z"]];
    let loser_moves = [["C", "Z"], ["A", "X"], ["B", "Y"]];

    let mut result_score = 3;
    for i in 0..3 {
        if their_move == winner_moves[i][0] && our_move == loser_moves[i][1] {
            result_score = 0;
        } else if our_move == winner_moves[i][1] && their_move == loser_moves[i][0] {
            result_score = 6;
        }
    }

    let move_score = match our_move {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => unimplemented!(),
    };

    move_score + result_score
}
pub fn part1() -> u32 {
    let input_text = utils::read_file("inputs/day2.txt");
    input_text.split('\n').map(score_round_part1).sum()
}


fn score_round_part2(round: &str) -> u32 {
    let round_parts = round.split(' ').collect::<Vec<&str>>();
    let their_move = round_parts[0];
    let result_goal = round_parts[1];

    // rock: A; paper: B; scissors: C
    let winner_moves = ["A", "B", "C"];
    let loser_moves = ["C", "A", "B"];

    let our_move = match (their_move, result_goal) {
        (tie, "Y") => tie,
        (lose, "X") => loser_moves[winner_moves.iter().position(|&t| t == lose).unwrap()],
        (win, "Z") => winner_moves[loser_moves.iter().position(|&t| t == win).unwrap()],
        (_, _) => unimplemented!()
    };
   

    let mut result_score = 3;
    for i in 0..3 {
        if their_move == winner_moves[i] && our_move == loser_moves[i] {
            result_score = 0;
        } else if our_move == winner_moves[i] && their_move == loser_moves[i] {
            result_score = 6;
        }
    }

    let move_score = match our_move {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _ => unimplemented!(),
    };

    move_score + result_score
}
pub fn part2() -> u32 {
    let input_text = utils::read_file("inputs/day2.txt");
    input_text.split('\n').map(score_round_part2).sum()
}
