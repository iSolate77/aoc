use anyhow::Result;
use std::collections::HashMap;

fn main() -> Result<()> {
    let lines = std::fs::read_to_string("src/bin/input1_1.prod")?;
    let sum: u32 = lines
        .lines()
        .filter_map(|line| get_calibration_value(line))
        .sum();

    println!("Sum: {}", sum);
    Ok(())
}

fn get_calibration_value(s: &str) -> Option<u32> {
    let word_to_digit = get_word_to_digit_map();
    let mut digits = Vec::new();
    let mut current_word = String::new();

    for c in s.chars() {
        if c.is_alphabetic() {
            current_word.push(c);
            // Check each spelled-out digit in the current word
            for (word, &digit) in &word_to_digit {
                if let Some(pos) = current_word.find(word) {
                    digits.push(digit);
                    current_word.replace_range(pos..pos + word.len(), "");
                }
            }
        } else if c.is_digit(10) {
            if !current_word.is_empty() {
                current_word.clear();
            }
            digits.push(c.to_digit(10).unwrap());
        }
    }

    // Check if a valid spelled-out digit is left at the end
    for (word, &digit) in &word_to_digit {
        if current_word == *word {
            digits.push(digit);
            break;
        }
    }

    // Determine the calibration value
    match digits.len() {
        1 => Some(digits[0] * 11),
        _ => digits
            .first()
            .zip(digits.last())
            .map(|(&first, &last)| first * 10 + last),
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
