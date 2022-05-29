use super::util::number_of_increases;

pub fn puzzle_two(data: &str) -> i32 {
    let numbers: Vec<i32> = data
        .split('\n')
        .map(|num_str| num_str.parse::<i32>().expect("Not a number."))
        .collect();

    let windowed_numbers: Vec<i32> = numbers
        .windows(3)
        .map(|window| window.iter().sum())
        .collect();

    number_of_increases(windowed_numbers)
}
