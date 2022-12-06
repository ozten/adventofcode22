use std::collections::HashSet;
use std::fs::read_to_string;

use std::error::Error;

const WINDOW_SIZE: usize = 4;

fn detect_signal(transmission: String) -> usize {
    let window_size = WINDOW_SIZE;
    let mut idx = 0;
    while idx + window_size < transmission.len() {
        let window: HashSet<char> = transmission[idx..idx + WINDOW_SIZE].chars().collect();
        if window.len() == WINDOW_SIZE {
            return idx + WINDOW_SIZE;
        }

        idx += 1;
    }
    0
}

#[test]
fn test_detect_signal() {
    assert_eq!(
        detect_signal(String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb")),
        7
    );
    assert_eq!(
        detect_signal(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz")),
        5
    );
    assert_eq!(
        detect_signal(String::from("nppdvjthqldpwncqszvftbrmjlhg")),
        6
    );
    assert_eq!(
        detect_signal(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")),
        10
    );
    assert_eq!(
        detect_signal(String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")),
        11
    );
}
fn main() -> Result<(), Box<dyn Error>> {
    let answer = detect_signal(read_to_string("src/input.txt")?);
    println!("Answer Part#1 {answer}");

    assert_eq!(answer, 1850);

    Ok(())
}
