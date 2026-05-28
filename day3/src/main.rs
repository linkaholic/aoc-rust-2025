#![allow(warnings)]

use std::{fs::File, io::prelude::*};

#[cfg(test)]
mod tests;

fn main() {
    let mut f = File::open("./data/input").expect("Error reading file");
    let mut content = String::new();
    let _ = f.read_to_string(&mut content);

    let mut total_joltage = 0;

    for line in content.lines() {
        total_joltage += get_largest_combination_part2(line, 12);
    }

    println!("{total_joltage}");
}

pub fn get_largest_combination_part2(line: &str, characters_total: usize) -> i64 {
    // println!("Scanning: \t{}", line);

    let character_count = line.chars().count();
    let mut largest_combination_str = String::new();
    let mut characters_missing = characters_total;

    let mut i = 0;
    while i < character_count {
        if characters_missing == 0 {
            break;
        }

        let max_index = (character_count - characters_missing);
        let current_subset = &line[i..=max_index];
        let (index, biggest) = get_biggest_number_in_subset(current_subset, i);
        largest_combination_str += &biggest.to_string();

        // println!("comparing {}", current_subset);
        // println!("index {} - value {}", index, biggest);

        i = index + 1;
        characters_missing -= 1;
    }

    // println!("Result:  \t{}", largest_combination_str);
    return largest_combination_str.parse::<i64>().unwrap();
}

pub fn get_biggest_number_in_subset(subset: &str, start_index: usize) -> (usize, u32) {
    subset
        .chars()
        .into_iter()
        .enumerate()
        .map(|(index, character)| {
            let parsed_number = character.to_digit(10).unwrap();
            let absolute_index = start_index + index;
            (absolute_index, parsed_number)
        })
        .max_by(|a, b| {
            let a_criteria = (a.1, b.0);
            let b_criteria = (b.1, a.0);
            a_criteria.cmp(&b_criteria)
        })
        .unwrap_or_default()
}

pub fn get_largest_combination_part1(line: &str) -> i64 {
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

    return largest_combination;
}
