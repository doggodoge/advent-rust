pub use puzzle_one::puzzle_one;
pub use puzzle_two::puzzle_two;

mod puzzle_one;
mod puzzle_two;
mod util;

pub fn print_solutions() {
    let data = include_str!("data/data.txt");

    println!("Sonar Sweep (day one) Solutions:");
    println!("\nPuzzle one: {}", puzzle_one(data));
    println!("\nPuzzle two: {}", puzzle_two(data));
}
