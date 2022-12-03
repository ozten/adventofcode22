use std::{fs::read_to_string, collections::HashMap};



fn priority(character: char) -> u32 {
  // can we use character range in match?
  let code_point = character as u32;
  match code_point {
    65..=90 => code_point - 38,
    97..=122 => code_point - 96,
    _ => panic!("Unexpected character for priority {:?}", character)
  }
}

#[test]
fn test_priority() {
    assert_eq!(priority('a'), 1);
    assert_eq!(priority('z'), 26);
    assert_eq!(priority('A'), 27);
    assert_eq!(priority('Z'), 52);
}

fn main() {
    let mut total = 0;
    for line in read_to_string("src/input.txt")
    .expect("Unable to read src/input.txt file")
    .split("\n") {

        println!("=== {:?} ====", line);

        let mut idx = 0;
        let length = line.len();
        let midpoint = length / 2;
        let mut items_seen: HashMap<char, bool> = HashMap::new();
        let mut errors: HashMap<char, bool> = HashMap::new();
        let mut chars = line.chars();

        if length == 0 {
            continue;
        }

        loop {
            // println!("Accessing {idx} of midpooint={midpoint} length={length}");
            let current_char = chars.nth(0).unwrap();
            // println!("{idx}: {current_char} {:?}", chars);
            if idx < midpoint {
                items_seen.insert(current_char, true);
            } else {
                match items_seen.get(&current_char) {
                    Some(_) => {
                        println!("Match! found {current_char}");
                        errors.insert(current_char, true);                        
                    },
                    None => {}
                }
            }
            idx += 1;
            if idx >= length {
                break;
            }
        }
        // read first half of the line
        // read second half of the line
        // find character that is common to both
        // take the character code of the letter 
        // sum them across all lines
        // println!("{}", line);

        for bad_item in errors.keys() {
            total += priority(*bad_item);
        }
        
    }

    let exp: &str = "vJrwpWtwJgWrhcsFMMfFFhFp";
    println!("Length of {exp} is {:?}", exp.len());
    println!("{:?}", priority(exp.chars().nth(0).unwrap()));
    println!("Total {total}");
    assert_eq!(total, 7446);
}
