use std::fs;

fn main() {
    let input_data = fs::read_to_string("src/input.txt").expect("couldn't open file");

    let mut top_calories: [i32; 3] = [0, 0, 0];
    let mut tmp_sum = 0;
    for line in input_data.lines() {
        if line == "" {
            if tmp_sum > top_calories[0] {
                top_calories.swap(1, 2);
                top_calories.swap(0, 1);
                top_calories[0] = tmp_sum;
            } else if tmp_sum > top_calories[1] && tmp_sum < top_calories[0] {
                top_calories.swap(1, 2);
                top_calories[1] = tmp_sum;
            } else if tmp_sum > top_calories[2] && tmp_sum < top_calories[1] {
                top_calories[2] = tmp_sum;
            }
            tmp_sum = 0;
            continue;
        }
        tmp_sum += line.parse::<i32>().unwrap();
    }
    let top_calories_sum = top_calories.iter().sum::<i32>();
    println!(
        "top 3 calories: {:#?} \n sum: {}",
        top_calories, top_calories_sum
    );
}
