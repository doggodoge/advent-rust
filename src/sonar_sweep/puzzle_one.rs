use super::util::number_of_increases;

pub fn puzzle_one(data: &str) -> i32 {
    let numbers = data
        .split('\n')
        .map(|num_str| num_str.parse::<i32>().expect("Not a number."))
        .collect();
    number_of_increases(numbers)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_return_seven_using_sample_data() {
        let data = include_str!("data/sample.txt");

        assert_eq!(7, puzzle_one(data));
    }
}
