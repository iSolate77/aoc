use anyhow::Result;

fn main() -> Result<()> {
    let mut max: Vec<usize> = include_str!("./input.text")
        .split("\n\n")
        .map(|x| {
            return x.lines().flat_map(str::parse::<usize>).sum();
        })
        .collect();
    max.sort_by(|a, b| b.cmp(a));
    println!("{:?}", max.into_iter().take(3).sum::<usize>());
    Ok(())
}
