fn main() {
    let instructions = vec![
        "move 1 from 2 to 1",
        "move 3 from 1 to 3",
        "move 2 from 2 to 1",
        "move 1 from 1 to 2",
    ];
    let result = solve(instructions);
    println!("The top crates of each stack are: {}", result);
}

fn solve(instructions: Vec<&str>) -> String {
    let mut stacks: Vec<Vec<String>> = vec![vec![], vec![], vec![]];

    for instruction in instructions {
        let parts: Vec<&str> = instruction.split(" ").collect();
        let quantity = parts[1].parse::<usize>().unwrap();
        let from_stack = parts[3].parse::<usize>().unwrap();
        let to_stack = parts[5].parse::<usize>().unwrap();
        for _ in 0..quantity {
            if let Some(crate_name) = stacks[from_stack - 1].pop() {
                stacks[to_stack - 1].push(crate_name);
            }
        }
    }
    let mut result = String::new();
    for i in 0..3 {
        if stacks[i].is_empty(){
            result.push_str("-")
        } else {
            result.push_str(stacks[i].last().unwrap());
        }
    }
    result
}
