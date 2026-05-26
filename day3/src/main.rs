use std::{fs::File, io::Read};

/*
The total output joltage is the sum of the maximum joltage from each bank,
so in this example, the total output joltage is 98 + 89 + 78 + 92 = 357.
*/

fn main() {
    let mut f = File::open("./data/sample").expect("Error reading file");
    let mut content = String::new();
    let _ = f.read_to_string(&mut content);

    let mut total_joltage = 0;
    for line in content.lines() {
        total_joltage += get_largest_combination(line);
    }

    println!("{total_joltage}");
}

fn get_largest_combination(line: &str) -> u32 {
    let mut largest_combination = 0;
    let (mut m_c1, mut m_c2) = (' ', ' ');
    for (i, c1) in line.chars().enumerate() {
        if i == (line.len() - 1) {
            break;
        }

        let j = i + 1;
        println!("{} - {}", i, j);
        let c2 = line.chars().nth(j).unwrap();
        let current_combination_str = format!("{}{}", c1, c2);
        let current_combination = current_combination_str.parse::<u32>().unwrap();
        if current_combination > largest_combination {
            m_c1 = c1;
            m_c2 = c2;
            largest_combination = current_combination;
        }
    }

    println!("largest combination {}-{}", m_c1, m_c2);
    return largest_combination;
}
