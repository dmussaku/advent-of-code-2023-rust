mod days;

use days::{day1};

fn main() {
    let input = day1::main::read_input("src/days/day1/input_files/input.txt");
    let result = day1::main::run_part_1(input);
    println!("Result: {}", result);
}
