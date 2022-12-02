use std::fs::read_to_string;

use sorted_list::SortedList;

pub fn elf_most_food() -> Vec<i64> {
    let mut sorted: SortedList<i64, i64> = SortedList::new();

    match read_to_string("src/input.txt") {
        Ok(input) => {
            let mut collect = 0;
            // let mut max_seen = 0;

            for value in input.split("\n").into_iter() {
                match value {
                    "" => {
                        sorted.insert(collect, collect);
                        collect = 0;
                    }
                    _ => match value.parse::<i64>() {
                        Ok(n) => {
                            collect = collect + n;
                        }
                        Err(error) => {
                            println!("Unable to parse {:?} as a number. {:?}", value, error);
                        }
                    },
                }
            }
        }
        Err(error) => {
            println!("Unable to load input {:?}", error);
        }
    }
    let len = sorted.len();
    let sl = sorted.keys().as_slice();

    let mut top: Vec<i64> = Vec::new();
    for el in sl[len - 3..len].iter() {
        top.push(*el);
    }

    top
}
