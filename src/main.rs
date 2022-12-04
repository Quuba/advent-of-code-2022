use std::fs;

fn main() {
    let input_data = fs::read_to_string("src/input.txt").unwrap();
    let mut point_sum = 0;
    for line in input_data.lines() {
        let (a, b) = line.split_at(1);
        let opponent_move: Move;
        let round_result: RoundResult;
        match a.trim() {
            "A" => opponent_move = Move::Rock,
            "B" => opponent_move = Move::Paper,
            "C" => opponent_move = Move::Scissors,
            _ => panic!(),
        }
        match b.trim() {
            "X" => round_result = RoundResult::Loss,
            "Y" => round_result = RoundResult::Draw,
            "Z" => round_result = RoundResult::Win,
            _ => panic!(),
        }
        let player_move: Move;
        match round_result {
            RoundResult::Win => match opponent_move {
                Move::Rock => player_move = Move::Paper,
                Move::Paper => player_move = Move::Scissors,
                Move::Scissors => player_move = Move::Rock,
            },
            RoundResult::Loss => match opponent_move {
                Move::Rock => player_move = Move::Scissors,
                Move::Paper => player_move = Move::Rock,
                Move::Scissors => player_move = Move::Paper,
            },
            RoundResult::Draw => {
                player_move = opponent_move.clone();
            }
        }

        point_sum += calculate_points(opponent_move, player_move)
    }
    println!("sum of points: {}", point_sum);
}

fn calculate_points(opponent_move: Move, your_move: Move) -> i32 {
    let mut points = 0;
    match your_move {
        Move::Rock => points += 1,
        Move::Paper => points += 2,
        Move::Scissors => points += 3,
    }
    if your_move == Move::Rock && opponent_move == Move::Scissors
        || your_move == Move::Paper && opponent_move == Move::Rock
        || your_move == Move::Scissors && opponent_move == Move::Paper
    {
        //you've won
        points += 6;
    } else if your_move == Move::Rock && opponent_move == Move::Paper
        || your_move == Move::Paper && opponent_move == Move::Scissors
        || your_move == Move::Scissors && opponent_move == Move::Rock
    {
        //opponent won
    } else if your_move == opponent_move {
        //draw
        points += 3;
    }
    return points;
}
#[derive(PartialEq, Eq, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}
enum RoundResult {
    Win,
    Loss,
    Draw,
}
