use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("src/bin/input3_1.test")?;
    let lines: Vec<&str> = input.lines().collect();

    let mut part_numbers = Vec::new();
    for i in 0..lines.len() {
        let current_line = lines[i];
        let previous_line = if i > 0 { lines[i - 1] } else { "" };
        let next_line = if i < lines.len() - 1 { lines[i + 1] } else { "" };

        process_line(previous_line, current_line, next_line, &mut part_numbers);
    }

    let total_sum: u32 = part_numbers.iter().sum();
    println!("Total sum of part numbers: {}", total_sum);

    Ok(())
}

fn process_line(previous_line: &str, current_line: &str, next_line: &str, part_numbers: &mut Vec<u32>) {
    let mut i = 0;
    while i < current_line.len() {
        let c = current_line.chars().nth(i).unwrap();
        if c.is_digit(10) {
            // Identify the whole number
            let mut num_str = String::new();
            while i < current_line.len() && current_line.chars().nth(i).unwrap().is_digit(10) {
                num_str.push(current_line.chars().nth(i).unwrap());
                i += 1;
            }
            let num = num_str.parse::<u32>().unwrap();

            // Check for adjacent symbols
            if is_adjacent_to_symbol(previous_line, current_line, next_line, i - num_str.len(), num_str.len()) {
                part_numbers.push(num);
            }
        } else {
            i += 1;
        }
    }
}

fn is_adjacent_to_symbol(previous_line: &str, current_line: &str, next_line: &str, start_index: usize, length: usize) -> bool {
    let end_index = start_index + length;
    let line_length = current_line.len();

    // Check adjacent cells in the current line
    if start_index > 0 && current_line.chars().nth(start_index - 1).unwrap() != '.' {
        return true;
    }
    if end_index < line_length && current_line.chars().nth(end_index).unwrap() != '.' {
        return true;
    }

    // Check adjacent cells in the previous line, if it exists
    if !previous_line.is_empty() {
        for i in start_index..=end_index {
            if i < previous_line.len() && previous_line.chars().nth(i).unwrap() != '.' {
                return true;
            }
        }
    }

    // Check adjacent cells in the next line, if it exists
    if !next_line.is_empty() {
        for i in start_index..=end_index {
            if i < next_line.len() && next_line.chars().nth(i).unwrap() != '.' {
                return true;
            }
        }
    }

    false
}
