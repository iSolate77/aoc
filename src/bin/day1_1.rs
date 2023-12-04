use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("src/bin/input1_1.prod");
    let sum: u32 = input?
        .lines()
        .filter_map(|line| get_calibration_value(line))
        .sum();

    println!("Sum: {}", sum);
    return Ok(());
}

fn get_calibration_value(line: &str) -> Option<u32> {
    let first_digit = line.chars().find_map(|c| c.to_digit(10));
    let last_digit = line.chars().rev().find_map(|c| c.to_digit(10));

    match (first_digit, last_digit) {
        (Some(first), Some(last)) => Some(first * 10 + last),
        _ => None,
    }
}
