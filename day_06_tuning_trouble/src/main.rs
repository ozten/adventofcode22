use std::collections::HashSet;
use std::fs::read_to_string;

use std::error::Error;

fn detect_signal(transmission: String, window_size: usize) -> usize {
    let mut idx = 0;
    while idx + window_size < transmission.len() {
        let window: HashSet<char> = transmission[idx..idx + window_size].chars().collect();
        if window.len() == window_size {
            return idx + window_size;
        }

        idx += 1;
    }
    0
}

#[test]
fn test_detect_signal() {
    assert_eq!(
        detect_signal(String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 4),
        7
    );
    assert_eq!(
        detect_signal(String::from("bvwbjplbgvbhsrlpgdmjqwftvncz"), 4),
        5
    );
    assert_eq!(
        detect_signal(String::from("nppdvjthqldpwncqszvftbrmjlhg"), 4),
        6
    );
    assert_eq!(
        detect_signal(String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 4),
        10
    );
    assert_eq!(
        detect_signal(String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 4),
        11
    );
}
fn main() -> Result<(), Box<dyn Error>> {
    let answer = detect_signal(read_to_string("src/input.txt")?, 4);
    println!("Answer Part#1 {answer}");

    assert_eq!(answer, 1850);

    Ok(())
}
