use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

enum Shapes {
    Rock,
    Paper,
    Scissors,
}

enum Outcomes {
    Lose,
    Draw,
    Win,
}

fn main() -> std::io::Result<()> {
    part1()?;
    part2()?;

    Ok(())
}

fn part1() -> std::io::Result<()> {
    let mut total_score = 0;

    if let Ok(lines) = read_lines("input/input.txt") {
        for line in lines {
            if let Ok(text) = line {
                let choices = text.split(" ").collect::<Vec<_>>();
                // println!("Round: {} + {}", choices[0], choices[1]);
                let opponent_shape = get_shape(choices[0]);
                let my_shape = get_shape(choices[1]);
                let outcome_score = outcome_score(&opponent_shape, &my_shape);
                let selection_score = selection_score(&my_shape);
                // println!("Outcome: {}, Selection: {}", outcome_score, selection_score);
                total_score += outcome_score + selection_score;
            }
        }
    }

    println!("Part 1: {total_score}");

    Ok(())
}

fn part2() -> std::io::Result<()> {
    let mut total_score = 0;

    if let Ok(lines) = read_lines("input/input.txt") {
        for line in lines {
            if let Ok(text) = line {
                let inputs = text.split(" ").collect::<Vec<_>>();
                let opponent_shape = get_shape(inputs[0]);
                let desired_outcome = get_outcome(inputs[1]);
                let my_shape = get_my_shape(&opponent_shape, &desired_outcome);
                let outcome_score = outcome_score(&opponent_shape, &my_shape);
                let selection_score = selection_score(&my_shape);
                total_score += outcome_score + selection_score;
            }
        }
    }

    println!("Part 2: {total_score}");

    Ok(())
}

fn get_shape(shape: &str) -> Shapes {
    match shape {
        "A" => Shapes::Rock,
        "B" => Shapes::Paper,
        "C" => Shapes::Scissors,
        "X" => Shapes::Rock,
        "Y" => Shapes::Paper,
        "Z" => Shapes::Scissors,
        &_ => panic!("Unknown shape: {}", shape),
    }
}

fn outcome_score(opponent: &Shapes, me: &Shapes) -> i32 {
    match (opponent, me) {
        (Shapes::Rock, Shapes::Rock) => 3,
        (Shapes::Rock, Shapes::Paper) => 6,
        (Shapes::Rock, Shapes::Scissors) => 0,
        (Shapes::Paper, Shapes::Rock) => 0,
        (Shapes::Paper, Shapes::Paper) => 3,
        (Shapes::Paper, Shapes::Scissors) => 6,
        (Shapes::Scissors, Shapes::Rock) => 6,
        (Shapes::Scissors, Shapes::Paper) => 0,
        (Shapes::Scissors, Shapes::Scissors) => 3,
    }
}

fn selection_score(selection: &Shapes) -> i32 {
    match selection {
        Shapes::Rock => 1,
        Shapes::Paper => 2,
        Shapes::Scissors => 3,
    }
}

fn get_outcome(outcome: &str) -> Outcomes {
    match outcome {
        "X" => Outcomes::Lose,
        "Y" => Outcomes::Draw,
        "Z" => Outcomes::Win,
        &_ => panic!("Unknown outcome: {}", outcome),
    }
}

fn get_my_shape(opponent: &Shapes, outcome: &Outcomes) -> Shapes {
    match (opponent, outcome) {
        (Shapes::Rock, Outcomes::Lose) => Shapes::Scissors,
        (Shapes::Rock, Outcomes::Draw) => Shapes::Rock,
        (Shapes::Rock, Outcomes::Win) => Shapes::Paper,
        (Shapes::Paper, Outcomes::Lose) => Shapes::Rock,
        (Shapes::Paper, Outcomes::Draw) => Shapes::Paper,
        (Shapes::Paper, Outcomes::Win) => Shapes::Scissors,
        (Shapes::Scissors, Outcomes::Lose) => Shapes::Paper,
        (Shapes::Scissors, Outcomes::Draw) => Shapes::Scissors,
        (Shapes::Scissors, Outcomes::Win) => Shapes::Rock,
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
