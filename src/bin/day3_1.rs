use anyhow::Result;
use std::collections::HashSet;

struct AoC {
    nums: Vec<PartNumber>,
    sy
}
struct PartNumber {
    value: i32,
    points: HashSet<(i32, i32)>,
}

impl PartNumber {
    fn new(row: i32, col: i32, ch: char) -> Self {
        let points = HashSet::from([
            (row - 1, col - 1), (row, col - 1), (row + 1, col - 1), // left hand side
            (row - 1, col), (row + 1, col),                          // abobe and below
            (row - 1, col + 1), (row, col + 1), (row + 1, col + 1),   // right hand side
        ]);
        Self {
            value: (ch as u8 - b'0') as i32,
            points,
        }
    }

    fn add_digit(&mut self, row: i32, col: i32, ch: char) {
        self.value = self.value * 10 + (ch as u8 - b'0') as i32;
        self.points
            .extend([(row - 1, col + 1), (row, col + 1), (row + 1, col + 1)]);
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("src/bin/input3_1.test")?;
    let lines: Vec<&str> = input.lines().collect();

    let mut cur_number: Option<PartNumber> = None;
    for (row, line) in lines.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                if let Some(ref mut num) = cur_number {
                    num.add_digit(row as i32, col as i32, ch);
                } else {
                    cur_number = Some(PartNumber::new(row as i32, col as i32, ch));
                }
            } else {
                if let Some(num) = cur_number.take() {
                    self.nums.push(num);
                                    
                }
            }
        }
    }

    Ok(())
}

