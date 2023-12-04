use anyhow::Result;
use std::collections::HashMap;

fn main() -> Result<()> {
    let lines = std::fs::read_to_string("src/bin/input1_2.test")?;
    let sum: u32 = lines
        .lines()
        .filter_map(|line| get_calibration_value(line))
        .sum();

    println!("Sum: {}", sum);
    Ok(())
}

fn get_calibration_value(line: &str) -> Option<u32> {
    let word_to_digit = get_word_to_digit_map();
    let mut digits = Vec::new();
    let mut current_word = String::new();
    for c in line.chars() {
        if c.is_alphabetic() {
            current_word.push(c);
            if let Some(&digit) = word_to_digit.get(current_word.as_str()) {
                digits.push(digit);
                current_word.clear();
            }
        } else if c.is_digit(10) {
            if !current_word.is_empty() {
                if let Some(&digit) = word_to_digit.get(current_word.as_str()) {
                    digits.push(digit);
                    current_word.clear();
                }
            }
        }
    }
    if digits.len() >= 2 {
        Some(digits[0] * 10 + digits[digits.len() - 1])
    } else {
        None
    }
}

fn get_word_to_digit_map() -> HashMap<&'static str, u32> {
    let mut map = HashMap::new();
    map.insert("one", 1);
    map.insert("two", 2);
    map.insert("three", 3);
    map.insert("four", 4);
    map.insert("five", 5);
    map.insert("six", 6);
    map.insert("seven", 7);
    map.insert("eight", 8);
    map.insert("nine", 9);
    map
}
