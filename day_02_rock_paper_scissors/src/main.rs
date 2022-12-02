use std::{collections::HashMap, fs::read_to_string};

type ShapeScore = u8;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
enum Choice {
    Rock(ShapeScore),
    Paper(ShapeScore),
    Scissors(ShapeScore),
}

fn get_choice_shape_score(choice: &Choice) -> ShapeScore {
    match choice {
        Choice::Rock(v) => *v,
        Choice::Paper(v) => *v,
        Choice::Scissors(v) => *v,
    }
}

const ROCK: Choice = Choice::Rock(1);
const PAPER: Choice = Choice::Paper(2);
const SCISSORS: Choice = Choice::Scissors(3);

type Outcome = u8;
const LOSS: Outcome = 0;
const TIE: Outcome = 3;
const WIN: Outcome = 6;

struct Move {
    choice: Choice,
}

fn compare(opponent_move: Move, player_move: &Move) -> Outcome {
    match (opponent_move.choice, &player_move.choice) {
        (x, &y) if x == y => TIE,
        (ROCK, &PAPER) | (PAPER, &SCISSORS) | (SCISSORS, &ROCK) => WIN,
        (ROCK, &SCISSORS) | (PAPER, &ROCK) | (SCISSORS, &PAPER) => LOSS,
        (a, b) => {
            panic!("Choice with an unknown score {:?} {:?}", a, b);
        }
    }
}

#[test]
fn test_compare() {
    assert_eq!(compare(Move { choice: ROCK }, &Move { choice: PAPER }), WIN);
    assert_eq!(
        compare(Move { choice: ROCK }, &Move { choice: SCISSORS }),
        LOSS
    );
    assert_eq!(compare(Move { choice: ROCK }, &Move { choice: ROCK }), TIE);

    assert_eq!(
        compare(Move { choice: PAPER }, &Move { choice: SCISSORS }),
        WIN
    );
    assert_eq!(compare(Move { choice: PAPER }, &Move { choice: ROCK }), LOSS);
    assert_eq!(compare(Move { choice: PAPER }, &Move { choice: PAPER }), TIE);
}

fn score(opponent_move: Move, player_move: &Move) -> u8 {
    let a = compare(opponent_move, player_move);
    let b = get_choice_shape_score(&player_move.choice);
    println!("==== Adding {:?} {:?}  ====", a, b);
    a + b
}

#[test]
fn test_score() {
    println!("===== YO Yo ====");
    assert_eq!(score(Move { choice: ROCK }, &Move { choice: PAPER }), 8);
    /*    assert_eq!(compare(Move{ choice: ROCK}, Move { choice: SCISSORS }), LOSS);
    assert_eq!(compare(Move{ choice: ROCK}, Move { choice: ROCK }), TIE);

    assert_eq!(compare(Move{ choice: PAPER}, Move { choice: SCISSORS }), WIN);
    assert_eq!(compare(Move{ choice: PAPER}, Move { choice: ROCK }), LOSS);
    assert_eq!(compare(Move{ choice: PAPER}, Move { choice: PAPER }), TIE);
     */
}

fn parse_moves(line:&str) -> (Move, Move) {
    let mut parts = line.split(" ");
    /*if parts.count() != 2 {
        panic!("Unexpected line format for {:?}", line);
    }*/
    println!("Looking at parts {:?}", parts);

    println!("Looking at parts {:?}", parts);


    let opp = match parts.next() {
        Some("A") => ROCK,
        Some("B") => PAPER,
        Some("C") => SCISSORS,
        _ => panic!("Malformed input")
    };
    let my = match parts.next() {
        Some("X") => ROCK,
        Some("Y") => PAPER,
        Some("Z") => SCISSORS,
        _ => panic!("Malformed input")
    };
    (Move { choice: opp }, Move { choice: my })
}

fn main() {
    let mut opponentCodes = HashMap::new();
    opponentCodes.insert(String::from("A"), ROCK);
    opponentCodes.insert(String::from("B"), PAPER);
    opponentCodes.insert(String::from("C"), SCISSORS);

    let mut playerCodes = HashMap::new();
    playerCodes.insert(String::from("X"), ROCK);
    playerCodes.insert(String::from("Y"), PAPER);
    playerCodes.insert(String::from("Z"), SCISSORS);

    let mut total: u64 = 0;
    for line in read_to_string("src/input.txt")
        .expect("Unable to read src/input.txt file")
        .split("\n") {
            if line.len() >= 3 {
              let (opp_move, my_move) = parse_moves(line);
              total = total + score(opp_move, &my_move) as u64;
            } else {
                println!("End of the line");
                println!("Total: {:?}", total);
            }
    }

    println!("Hello, world!");
}
