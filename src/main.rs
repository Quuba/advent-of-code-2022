use std::fs;

fn main() {
    let input_data = fs::read_to_string("src/input.txt").unwrap();
    let mut point_sum = 0;
    for line in input_data.lines() {
        let (a, b) = line.split_at(1);
        let player_move: Move;
        let opponent_move: Move;
        match a.trim() {
            "A" => opponent_move = Move::Rock,
            "B" => opponent_move = Move::Paper,
            "C" => opponent_move = Move::Scissors,
            _ => panic!(),
        }
        match b.trim() {
            "X" => player_move = Move::Rock,
            "Y" => player_move = Move::Paper,
            "Z" => player_move = Move::Scissors,
            _ => panic!(),
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
    } else if your_move == opponent_move{
        //draw
        points += 3;
    }
    return points;
}
#[derive(PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}
