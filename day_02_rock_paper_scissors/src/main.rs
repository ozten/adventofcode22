use std::fs::read_to_string;

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

enum Strategy {
    MoveValue,
    Outcome,
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
    assert_eq!(
        compare(Move { choice: PAPER }, &Move { choice: ROCK }),
        LOSS
    );
    assert_eq!(
        compare(Move { choice: PAPER }, &Move { choice: PAPER }),
        TIE
    );
}

fn score(opponent_move: Move, player_move: &Move) -> u8 {
    let a = compare(opponent_move, player_move);
    let b = get_choice_shape_score(&player_move.choice);
    a + b
}

#[test]
fn test_score() {
    assert_eq!(score(Move { choice: ROCK }, &Move { choice: PAPER }), 8);
}

fn parse_moves(line: &str, strategy: Strategy) -> (Move, Move) {
    let mut parts = line.split(" ");

    let opp = match parts.next() {
        Some("A") => ROCK,
        Some("B") => PAPER,
        Some("C") => SCISSORS,
        _ => panic!("Malformed input"),
    };
    let my = match strategy {
        Strategy::MoveValue => match parts.next() {
            Some("X") => ROCK,
            Some("Y") => PAPER,
            Some("Z") => SCISSORS,
            _ => panic!("Malformed input"),
        },
        Strategy::Outcome => match parts.next() {
            Some("X") => move_by_outcome(opp, LOSS),
            Some("Y") => move_by_outcome(opp, TIE),
            Some("Z") => move_by_outcome(opp, WIN),
            _ => panic!("Malformed input"),
        },
    };

    (Move { choice: opp }, Move { choice: my })
}

fn move_by_outcome(opponent_choice: Choice, desired_outcome: Outcome) -> Choice {
    match (opponent_choice, desired_outcome) {
        (_, TIE) => opponent_choice,
        (ROCK, LOSS) => SCISSORS,
        (PAPER, LOSS) => ROCK,
        (SCISSORS, LOSS) => PAPER,
        (ROCK, WIN) => PAPER,
        (PAPER, WIN) => SCISSORS,
        (SCISSORS, WIN) => ROCK,
        _ => panic!("Unknown state {:?} {:?}", opponent_choice, desired_outcome),
    }
}

fn main() {
    let mut part_1_total: u64 = 0;
    let mut part_2_total: u64 = 0;
    for line in read_to_string("src/input.txt")
        .expect("Unable to read src/input.txt file")
        .split("\n")
    {
        if line.len() >= 3 {
            let (opp_move, part_1_move) = parse_moves(line, Strategy::MoveValue);
            part_1_total = part_1_total + score(opp_move, &part_1_move) as u64;
            let (opp_move, part_2_move) = parse_moves(line, Strategy::Outcome);
            part_2_total = part_2_total + score(opp_move, &part_2_move) as u64;
        } else {
            // end of the input
            assert_eq!(part_1_total, 9759);
            println!("Answer 1 Total: {:?}", part_1_total);

            assert_eq!(part_2_total, 12429);
            println!("Answer 2 Total: {:?}", part_2_total);
        }
    }
}
