use std::fs;

fn main() {
    let mut stacks: Vec<Vec<char>> = vec![];

    let input_file = fs::read_to_string("src/input.txt").unwrap();
    let lines = input_file.lines().collect::<Vec<&str>>();
    let tmp_line = lines[0].clone().chars();
    let stack_count = (tmp_line.collect::<Vec<char>>().len() + 1) / 4;
    println!("stacks: {stack_count}");

    for _ in 0..stack_count {
        stacks.push(vec![]);
    }

    let mut max_height = 0;
    let mut tmp_lines = lines.iter();
    while tmp_lines.next().unwrap().chars().next().unwrap() == '[' {
        max_height += 1;
    }

    println!("tallest stack: {max_height}");

    //TODO: read and save crates
    //TODO: skip the pictogram at the top of input.txt file
    //TODO: read moves and move crates
    //TODO: print the result
}

// fn move_crate<T: Clone>(from: &mut Vec<T>, to: &mut Vec<T>) {
//     let tmp = &from.pop().unwrap();
//     to.push(tmp.clone());
// }
// fn move_crate_repeat<T: Clone>(from: &mut Vec<T>, to: &mut Vec<T>, repeat: usize) {
//     for i in 0..repeat {
//         let tmp = &from.pop().unwrap();
//         to.push(tmp.clone());
//     }
// }
