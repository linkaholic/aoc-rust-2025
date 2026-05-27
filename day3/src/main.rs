use std::{fs::File, io::Read};

/*
The total output joltage is the sum of the maximum joltage from each bank,
so in this example, the total output joltage is 98 + 89 + 78 + 92 = 357.
*/

fn main() {
    let mut f = File::open("./data/input").expect("Error reading file");
    let mut content = String::new();
    let _ = f.read_to_string(&mut content);

    let mut total_joltage = 0;
    for line in content.lines() {
        total_joltage += get_largest_combination(line);
    }

    println!("{total_joltage}");
}

fn get_largest_combination(line: &str) -> i64 {
    let mut largest_combination = 0;
    let total_characters = line.chars().count();
    for (i, c1) in line.chars().enumerate().take(total_characters - 1) {
        let current_largest_combination: i64 = line
            .chars()
            .zip(std::iter::repeat(c1))
            .enumerate()
            .filter(|combination| {
                if i == combination.0 || i > combination.0 {
                    false
                } else {
                    true
                }
            })
            .map(|combination| {
                let (c1, c2) = combination.1;
                let current_combination = format!("{}{}", c2, c1);
                current_combination.parse::<i64>().unwrap()
            })
            .max()
            .unwrap();

        if current_largest_combination > largest_combination {
            largest_combination = current_largest_combination;
        }
    }

    // println!("line: {}", line);
    // println!("\tcombination: {}", largest_combination);
    return largest_combination;
}
