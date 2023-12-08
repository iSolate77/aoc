use anyhow::{anyhow, Result};
use std::str::FromStr;

#[derive(Debug)]
struct Game {
    id: u32,
    blue: u32,
    red: u32,
    green: u32,
}

impl Game {
    fn new(id: u32) -> Self {
        Self {
            id,
            blue: 0,
            red: 0,
            green: 0,
        }
    }

    fn add_counts(&mut self, blue: u32, red: u32, green: u32) {
        self.blue += blue;
        self.red += red;
        self.green += green;
    }

    // fn impossible(&self) -> bool {
    //     self.red > 12 || self.green > 13 || self.blue > 14
    // }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(':');
        let id_str = parts
            .next()
            .ok_or_else(|| anyhow!("Missing game ID"))?
            .trim()
            .strip_prefix("Game")
            .ok_or_else(|| anyhow!("Invalid game ID format"))?
            .trim();
        let id: u32 = id_str.parse()?;

        let mut game = Game::new(id);

        if let Some(tries) = parts.next() {
            for try_str in tries.split(';') {
                let mut blue = 0;
                let mut red = 0;
                let mut green = 0;

                for color_part in try_str.split(',') {
                    let mut parts = color_part.trim().split_whitespace();

                    let count_str = parts
                        .next()
                        .ok_or_else(|| anyhow!("Missing count in color part"))?;
                    let count: u32 = count_str.parse()?;

                    let color_name = parts
                        .next()
                        .ok_or_else(|| anyhow!("Missing color name in color part"))?;

                    match color_name {
                        "blue" => blue = count,
                        "red" => red = count,
                        "green" => green = count,
                        _ => return Err(anyhow!("Invalid color name {}", color_name)),
                    };
                }
                if blue > 14 || red > 12 || green > 13 {
                    return Ok(Game {
                        id,
                        blue: 0,
                        red: 0,
                        green: 0,
                    });
                }
                game.add_counts(blue, red, green);
            }
        }

        Ok(game)
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("src/bin/input2_1.prod")?;
    println!("{:?}", read_and_process_games(input));
    Ok(())
}

fn read_and_process_games(input: String) -> Result<u32> {
    let mut sum_of_ids = 0;
    for line in input.lines() {
        let game = Game::from_str(line)?;
        if game.blue == 0 && game.red == 0 && game.green == 0 {
            // Skip this game as it's impossible
            continue;
        }
        sum_of_ids += game.id;
    }
    Ok(sum_of_ids)
}
