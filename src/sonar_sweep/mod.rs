pub use puzzle_one::puzzle_one;
pub use puzzle_two::puzzle_two;

use crate::util::time::time;

mod puzzle_one;
mod puzzle_two;
mod util;

pub fn print_solutions() {
    let data = include_str!("data/data.txt");

    let (puzzle_one_result, puzzle_one_duration) = time(|| puzzle_one(data));
    let (puzzle_two_result, puzzle_two_duration) = time(|| puzzle_two(data));

    let total_duration: std::time::Duration = puzzle_one_duration + puzzle_two_duration;

    println!("Sonar Sweep (day one) Solutions:");
    println!(
        "\tPuzzle one: {} ({}μs)",
        puzzle_one_result,
        puzzle_one_duration.as_micros()
    );
    println!(
        "\tPuzzle two: {} ({}μs)",
        puzzle_two_result,
        puzzle_two_duration.as_micros()
    );
    println!("\tTotal Duration: {}μs", total_duration.as_micros());
}
