use std::fs;

fn main() {
    let input_file = fs::read_to_string("src/input.txt").unwrap();

    let lines = input_file.lines().collect::<Vec<&str>>();
    let tmp_line = lines[0].clone().chars();
    let stack_count = (tmp_line.collect::<Vec<char>>().len() + 1) / 4;
    println!("stacks: {stack_count}");

    //initialize variable that holds the crate stacks
    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in 0..stack_count {
        stacks.push(vec![]);
    }

    //calculate height of the tallest stack
    let mut max_height = 0;
    let mut tmp_lines = lines.iter();
    while !tmp_lines.next().unwrap().starts_with(" 1") {
        max_height += 1;
    }

    println!("tallest stack: {max_height}");

    //read and save crates in memory
    for i in 0..stack_count {
        for j in (0..max_height).rev() {
            let tmp_char = lines[j].clone().chars().collect::<Vec<char>>()[1 + 4 * i];
            if tmp_char == ' ' {
                break;
            }
            stacks[i].push(tmp_char);
        }
    }

    //skip the diagram at the top of the input file
    let mut tmp_lines = lines.clone();
    for _ in 0..max_height + 2 {
        tmp_lines.remove(0);
    }

    //read moves and move crates
    for i in 0..tmp_lines.len() {
        let line = tmp_lines[i].split(' ').collect::<Vec<&str>>();
        let from_stack_index = line[3].to_string().parse::<usize>().unwrap() - 1;
        let to_stack_index = line[5].to_string().parse::<usize>().unwrap() - 1;

        for _ in 0..line[1].to_string().parse::<usize>().unwrap() {
            let tmp = stacks[from_stack_index].pop().unwrap();
            stacks[to_stack_index].push(tmp);
        }
    }

    //print the result
    let mut res = String::new();
    for i in 0..stacks.len() {
        res.push(stacks[i].last().unwrap().to_owned());
    }
    println!("{res}");
}
