use std::fs;

fn main() {
    let input_data = fs::read_to_string("src/input.txt").expect("couldn't open file");
    let input_chars = input_data.chars().collect::<Vec<char>>();
    let mut final_val = 0;

    let mut buffer: [char; 4] = [' '; 4];
    for i in 0..4 {
        buffer[i] = input_chars[i];
    }

    if is_buffer_unique(&buffer) {
        final_val = 4;
        println!("{final_val}");
        return;
    }

    for i in 4..input_chars.len() {
        shift_buffer(&mut buffer, input_chars[i]);
        if is_buffer_unique(&buffer) {
            final_val = i + 1;
            break;
        }
    }

    println!("{final_val}");
}
fn shift_buffer(buffer: &mut [char; 4], new_char: char) {
    for i in 0..3 {
        buffer[i] = buffer[i + 1];
    }
    buffer[3] = new_char;
}

fn is_buffer_unique(buffer: &[char; 4]) -> bool {
    for i in 0..3 {
        for j in 0..4 {
            if j == i {
                continue;
            }
            if buffer[i] == buffer[j] {
                return false;
            }
        }
    }
    return true;
}
