//Opponent moves: 1º column
//A - rock
//B - Paper
//C - Scissors
//2º column: X, Y, Z self move.

//total score = sum of all scores for each round.
//score round = round result + shape score
//score: rock 1, paper 2, scissor 3.
//points: 0 loss, 3 draw, 6 won

//PART 2:
// X = loss
// Y = draw
// Z = win

use std::path::Path;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Movement {
    Rock,
    Paper,
    Scissors,
}

const MOVES: [Movement; 3] = [Movement::Rock, Movement::Paper, Movement::Scissors];

impl FromStr for Movement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Rock),
            "X" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "Y" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

impl From<Movement> for &str {
    fn from(m: Movement) -> Self {
        match m {
            Movement::Rock => "X",
            Movement::Paper => "Y",
            Movement::Scissors => "Z",
        }
    }
}

impl Ord for Movement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Movement::Rock, Movement::Rock) => std::cmp::Ordering::Equal,
            (Movement::Paper, Movement::Paper) => std::cmp::Ordering::Equal,
            (Movement::Scissors, Movement::Scissors) => std::cmp::Ordering::Equal,
            (Movement::Rock, Movement::Paper) => std::cmp::Ordering::Less,
            (Movement::Rock, Movement::Scissors) => std::cmp::Ordering::Greater,
            (Movement::Paper, Movement::Rock) => std::cmp::Ordering::Greater,
            (Movement::Paper, Movement::Scissors) => std::cmp::Ordering::Less,
            (Movement::Scissors, Movement::Rock) => std::cmp::Ordering::Less,
            (Movement::Scissors, Movement::Paper) => std::cmp::Ordering::Greater,
        }
    }
}

// STACK OVERFLOW
/*impl PartialEq for Movement {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
} */

impl PartialOrd for Movement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    if let Ok(lines) = read_lines("./input/input.txt") {
        let mut score: u32 = 0;
        let mut score_pt2: u32 = 0;
        for line in lines {
            if let Ok(line_movs) = line {
                let movements: Vec<&str> = line_movs.split_whitespace().collect();

                let mov_opponent = movements[0].parse().unwrap();
                let mov_my = movements[1].parse().unwrap();

                let result = result_score(mov_my, mov_opponent);

                let my_score = movement_score(movements[1].parse().unwrap());
                score = score + result + my_score;

                //pt 2
                let target_move: &str = mov_my.into();
                let pick_move = match target_move {
                    //map loss to shape
                    "X" => MOVES.iter().cloned().find(|p| p.lt(&mov_opponent)),
                    //map draw to shape
                    "Y" => MOVES.iter().cloned().find(|p| p.eq(&mov_opponent)),
                    //map win to shape
                    "Z" => MOVES.iter().cloned().find(|p| p.gt(&mov_opponent)),
                    &_ => None,
                };

                println!(
                    "Opponent mov: {:?}, target move: {}, should pick mov: {:?}",
                    mov_opponent, target_move, pick_move
                );
                let result = result_score(pick_move.unwrap(), mov_opponent);
                score_pt2 = score_pt2 + result + movement_score(pick_move.unwrap());
                println!("Score pt2: {}, result score: {}", score_pt2, result);
            }
        }

        println!("Score: {}", score);
        println!("Score pt2: {}", score_pt2);
    }
}

fn movement_score(movement: Movement) -> u32 {
    match movement {
        Movement::Rock => 1,
        Movement::Paper => 2,
        Movement::Scissors => 3,
    }
}

fn result_score(my_mov: Movement, opp_mov: Movement) -> u32 {
    match opp_mov.cmp(&my_mov) {
        std::cmp::Ordering::Less => 6,
        std::cmp::Ordering::Greater => 0,
        std::cmp::Ordering::Equal => 3,
    }
}

fn read_lines<P>(path: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}
