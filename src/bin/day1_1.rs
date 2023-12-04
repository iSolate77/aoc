use anyhow::Result;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input1_1.test");
    println!("{:?}", input?);
    return Ok(());
}
