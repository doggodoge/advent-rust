pub fn number_of_increases(pings: Vec<i32>) -> i32 {
    let mut number_of_increases = 0;
    pings
        .into_iter()
        .reduce(|prev, current| {
            if current > prev {
                number_of_increases += 1;
            }
            current
        })
        .expect("Failed to get number of increases");
    number_of_increases
}
