use std::fs;

fn main() {
    let input_data = fs::read_to_string("src/input.txt").unwrap();
    let mut repeating_items: Vec<char> = vec![];
    for line in input_data.lines() {
        let ruck_sack: RuckSack = RuckSack::from_string(line.to_string());
        let item = ruck_sack.get_duplicated_item().unwrap();
        repeating_items.push(item);
    }

    let mut sum = 0;
    for item in repeating_items{
        sum += get_item_priority(item).unwrap();
    }
    println!("sum: {}", sum);
}
fn get_item_priority(c:char) -> Result<i32, ()>{
    if c.is_ascii_lowercase(){
        return Ok(c as u32 as i32 - 96);
    }else if c.is_ascii_uppercase(){
        return Ok(c as u32 as i32 - 65 + 27);
    }
    return Err(());
}

struct RuckSack {
    pub first_compartment: Vec<char>,
    pub second_compartment: Vec<char>,
}

impl RuckSack {
    pub fn from_string(data: String) -> Self {
        let (a, b) = data.split_at(data.len() / 2);

        RuckSack {
            first_compartment: a.chars().collect::<Vec<char>>(),
            second_compartment: b.chars().collect::<Vec<char>>(),
        }
    }

    pub fn get_duplicated_item(self) -> Result<char, ()> {
        for c in self.first_compartment {
            if self.second_compartment.contains(&c) {
                return Ok(c.clone());
            }
        }
        return Err(());
    }
}
