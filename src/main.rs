use std::fs;

fn main() {
    let input_data = fs::read_to_string("src/input.txt").expect("couldn't open file");

    let mut first_min;
    let mut first_max;
    let mut second_min;
    let mut second_max;
    let mut first_range;
    let mut second_range;
    let mut a;
    let mut b;
    let mut counter = 0;

    for line in input_data.lines() {
        (first_range, second_range) = line.trim().split_at(line.find(',').unwrap());
        (a, b) = first_range.split_at(first_range.find('-').unwrap());

        let filtered = b.chars().filter(|&c| c != '-').collect::<String>();
        b = &filtered;
        
        first_min = a.parse::<i32>().unwrap();
        first_max = b.parse::<i32>().unwrap();

        let filtered = second_range
            .chars()
            .filter(|&c| c != ',')
            .collect::<String>();
        second_range = &filtered;

        (a, b) = second_range.split_at(second_range.find('-').unwrap());
        
        let filtered = b.chars().filter(|&c| c != '-').collect::<String>();
        b = &filtered;
        
        second_min = a.parse::<i32>().unwrap();
        second_max = b.parse::<i32>().unwrap();

        let overlaps = !(first_max < second_min || second_max < first_min);
        if overlaps {
            counter += 1;
        }
    }
    println!("count: {}", counter);
}
