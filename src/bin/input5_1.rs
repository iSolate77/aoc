use anyhow::Result;
/*
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
*/
fn main() -> Result<()> {
    let input = std::fs::read_to_string("./input5_1.test");
    println!("{:?}", input?.split("\n\n"));
    return Ok(());
}

