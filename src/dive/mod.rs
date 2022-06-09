use crate::util::time::time;

mod puzzle_one;
mod puzzle_two;

use puzzle_one::puzzle_one;

pub fn print_solutions() {
    let data: Vec<String> = include_str!("data/data.txt")
        .split("\n")
        .map(|line| line.to_string())
        .collect();

    let (puzzle_one_result, puzzle_one_duration) = time(|| puzzle_one(&data));

    let total_duration: std::time::Duration = puzzle_one_duration;

    println!("Sonar Sweep (day one) Solutions:");
    println!(
        "\tPuzzle one: {} ({}μs)",
        puzzle_one_result,
        puzzle_one_duration.as_micros()
    );
    println!("\tTotal Duration: {}μs", total_duration.as_micros());
}
