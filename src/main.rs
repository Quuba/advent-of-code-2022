use std::fs;

fn main() {
    let input_data = fs::read_to_string("src/input.txt").expect("couldn't open file");

    let mut max_calories = 0;
    let mut tmp_sum = 0;
    for line in input_data.lines() {
        if line == "" {
            if tmp_sum > max_calories {
                max_calories = tmp_sum;
            }
            tmp_sum = 0;
            continue;
        }
        tmp_sum += line.parse::<i32>().unwrap();
    }
    println!("most calories: {}", max_calories);
}
