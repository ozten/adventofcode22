mod day1;
use day1::elf_most_food;

const TOP_ANSWER: usize = 2;
fn main() {
    let top_calories = elf_most_food();

    println!("Answer #1 {:?}", top_calories[TOP_ANSWER]);
    println!("Answer #2 {:?}", top_calories.iter().sum::<i64>());
}
