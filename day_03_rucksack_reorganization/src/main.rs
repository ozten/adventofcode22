use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn priority(character: char) -> u32 {
    // can we use character range in match?
    let code_point = character as u32;
    match code_point {
        65..=90 => code_point - 38,
        97..=122 => code_point - 96,
        _ => panic!("Unexpected character for priority {:?}", character),
    }
}

#[test]
fn test_priority() {
    assert_eq!(priority('a'), 1);
    assert_eq!(priority('z'), 26);
    assert_eq!(priority('A'), 27);
    assert_eq!(priority('Z'), 52);
}

fn detect_badge_type(sack_groups: &Vec<&str>) -> char {
    let mut uniq_sack_groups: Vec<HashSet<char>> = Vec::new();
    for sack in sack_groups {
        let mut uniq_chars = HashSet::new();

        for char in sack.chars() {
            uniq_chars.insert(char);
        }
        uniq_sack_groups.push(uniq_chars.clone());
    }
    let mut sack_iter = uniq_sack_groups.iter();
    let a = sack_iter.next().unwrap();
    let b = sack_iter.next().unwrap();
    let c = sack_iter.next().unwrap();

    // Which item_type is in all three Rucksacks a, b, and c?
    let mut a_b_common_items = a.intersection(&b);
    *a_b_common_items.find(|item_type| c.contains(item_type)).unwrap()
}

fn main() {
    let mut total = 0;
    // Store rucksacks in groups of three
    let mut sack_groups: Vec<&str> = Vec::new();
    let mut badge_priorities = 0;

    for line in read_to_string("src/input.txt")
        .expect("Unable to read src/input.txt file")
        .split("\n")
    {
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
            let current_char = chars.nth(0).unwrap();
            if idx < midpoint {
                items_seen.insert(current_char, true);
            } else {
                match items_seen.get(&current_char) {
                    Some(_) => {
                        errors.insert(current_char, true);
                    }
                    None => {}
                }
            }
            idx += 1;
            if idx >= length {
                break;
            }
        }

        for bad_item in errors.keys() {
            total += priority(*bad_item);
        }

        // Part 2
        sack_groups.push(line);
        if sack_groups.len() == 3 {
            let badge_type = detect_badge_type(&sack_groups);
            badge_priorities += priority(badge_type);
            sack_groups.clear();
        }
    }
    println!("Total {total}");
    assert_eq!(total, 7446);

    println!("Part 2 Total {badge_priorities}");
    assert_eq!(badge_priorities, 2646);
}
