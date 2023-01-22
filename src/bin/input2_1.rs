use std::collections::HashMap;

use anyhow::Result;

fn get_hashmap() -> HashMap<&'static str, i32> {
    let mut map = HashMap::new();

    map.insert("A X", 1 + 3);
    map.insert("A Y", 2 + 6);
    map.insert("A Z", 3 + 0);
    map.insert("B X", 1 + 0);
    map.insert("B Y", 2 + 3);
    map.insert("B Z", 3 + 6);
    map.insert("C X", 1 + 6);
    map.insert("C Y", 2 + 0);
    map.insert("C Z", 3 + 3);

    return map;
}

fn main() -> Result<()> {
    let input = include_str!("./input2.txt");
    let lines: Vec<&str> = input.split('\n').collect();

    let hm = get_hashmap();
    
    let mut sum = 0;
    for x in lines {
        if let Some(score) = hm.get(x) {
            sum += score;
        }
    }

    println!("lines: {:?}", sum);

    Ok(())
}
