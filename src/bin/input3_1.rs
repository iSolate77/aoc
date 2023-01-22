fn main() {
    let input = include_str!("./test3_2.txt");
    let lines: Vec<&str> = input.lines().collect();
    let mut common = String::new();
    for i in 0..lines.len() {
        let middle = lines[i].len() / 2;
        let split = lines[i].split_at(middle);
        for j in split.0.chars() {
            if split.1.contains(j) {
                common.push(j);
                break;
            }
        }
    }
    println!("Common: {}", common);
    // find ascci value of each char in common
    let mut sum = 0;
    for i in common.chars() {
        if i.is_lowercase() {
            sum += i as u32 - 96;
        } else {
            sum += i as u32 - 38 ;
        }
    }
    println!("Sum: {}", sum);
}
