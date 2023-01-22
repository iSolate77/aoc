use anyhow::Result;
use std::str::FromStr;

struct Task {
    start: usize,
    end: usize,
}

impl Task {
    pub fn contains_some(&self, other: &Task) -> bool {
        return self.start <= other.start && self.end >= other.start
            || self.end >= other.end && self.start <= other.end;
    }
}
struct Tasks {
    left: Task,
    right: Task,
}
impl Tasks {
    pub fn one_task_contains_some_of_the_other(&self) -> bool {
        return self.left.contains_some(&self.right) || self.right.contains_some(&self.left);
    }
}
impl FromStr for Tasks {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(',').expect("aoc is not a liar");
        return Ok(Tasks {
            left: left.parse()?,
            right: right.parse()?,
        });
    }
}

impl FromStr for Task {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once('-').expect("AoC is not playing");
        return Ok(Task {
            start: a.parse()?,
            end: b.parse()?,
        });
    }
}

fn main() -> Result<()> {
    println!(
        "let's do this: {}",
        std::fs::read_to_string("./input.test")?
            .lines()
            .flat_map(|line| line.parse::<Tasks>())
            .filter(|tasks| tasks.one_task_contains_some_of_the_other())
            .count()
    );
    return Ok(());
}
