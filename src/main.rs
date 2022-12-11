use std::fs;

fn main() {
    let input_data = fs::read_to_string("src/input.txt").unwrap();
    let mut badges: Vec<char> = vec![];
    let mut arr: [&str; 3] = [""; 3];
    let mut counter = 0;
    for line in input_data.lines() {
        arr[counter] = line;
        if counter == 2 {
            badges.push(find_repeating_character(&arr).unwrap());
            counter = 0;
        }else{
            counter += 1;
        }
    }

    let mut sum = 0;
    for item in badges {
        sum += get_item_priority(item).unwrap();
    }
    println!("sum: {}", sum);
}

fn find_repeating_character(arr: &[&str; 3]) -> Result<char, ()> {
    for c in arr[0].chars() {
        if arr[1].contains(c) && arr[2].contains(c) {
            return Ok(c);
        }
    }
    return Err(());
}

fn get_item_priority(c: char) -> Result<i32, ()> {
    if c.is_ascii_lowercase() {
        return Ok(c as u32 as i32 - 96);
    } else if c.is_ascii_uppercase() {
        return Ok(c as u32 as i32 - 65 + 27);
    }
    return Err(());
}

// struct RuckSack {
//     pub first_compartment: Vec<char>,
//     pub second_compartment: Vec<char>,
// }

// impl RuckSack {
//     pub fn from_string(data: String) -> Self {
//         let (a, b) = data.split_at(data.len() / 2);

//         RuckSack {
//             first_compartment: a.chars().collect::<Vec<char>>(),
//             second_compartment: b.chars().collect::<Vec<char>>(),
//         }
//     }

//     pub fn get_duplicated_item(self) -> Result<char, ()> {
//         for c in self.first_compartment {
//             if self.second_compartment.contains(&c) {
//                 return Ok(c.clone());
//             }
//         }
//         return Err(());
//     }
// }
