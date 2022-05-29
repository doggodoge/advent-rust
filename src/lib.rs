pub mod sonar_sweep;

pub fn time<F, R>(func: F) -> (R, std::time::Duration)
where
    F: Fn() -> R,
{
    let start = std::time::Instant::now();
    let result = func();
    let end = std::time::Instant::now();
    let duration = end - start;
    (result, duration)
}
