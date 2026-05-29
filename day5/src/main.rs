use std::{fs::File, io::Read};

#[cfg(test)]
mod tests;

fn main() {
    let mut f = File::open("./data/input").expect("Error reading file");
    let mut content = String::new();
    let _ = f.read_to_string(&mut content);

    fresh_ingredients_part2(content);
}

fn fresh_ingredients_part2(content: String) {
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut fresh_ingredients = 0;

    for line in content.lines() {
        let line_type = LineType::get_line_type(line);
        match line_type {
            Some(LineType::Range) => ranges.push(get_ranges(line)),
            Some(LineType::Id) => {}
            None => {}
        }
    }

    ranges.sort_by_key(|&(start, _)| start);
    let merged_ranges = merge_ranges(ranges);

    merged_ranges
        .iter()
        .for_each(|&(start, end)| fresh_ingredients += (end - start) + 1);

    println!("{fresh_ingredients}");
}

fn merge_ranges(ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut merged_ranges: Vec<(i64, i64)> = Vec::new();
    for (new_start, new_end) in ranges {
        let mut merged = false;

        for i in 0..merged_ranges.len() {
            let (old_start, old_end) = merged_ranges[i];
            if old_end >= new_start {
                merged_ranges[i] = (old_start, old_end.max(new_end));

                merged = true;
                break;
            }
        }

        if !merged {
            merged_ranges.push((new_start, new_end));
        }
    }

    merged_ranges
}

fn get_ranges(line: &str) -> (i64, i64) {
    let mut split = line.split("-");

    let (start, end) = (
        split.next().unwrap().parse::<i64>().unwrap(),
        split.next().unwrap().parse::<i64>().unwrap(),
    );

    (start, end)
}

enum LineType {
    Range,
    Id,
}

trait LineTypeCharacter {
    fn get_line_type(line: &str) -> Option<LineType>;
}

impl LineTypeCharacter for LineType {
    fn get_line_type(line: &str) -> Option<LineType> {
        if line.contains("-") {
            return Some(LineType::Range);
        }

        if line.len() != 0 {
            return Some(LineType::Id);
        };

        None
    }
}

fn fresh_ingredients_part1(content: String) {
    let mut ranges: Vec<(i64, i64)> = Vec::new();
    let mut ingredient_ids: Vec<i64> = Vec::new();

    for line in content.lines() {
        let line_type = LineType::get_line_type(line);
        match line_type {
            Some(LineType::Id) => ingredient_ids.push(line.parse::<i64>().unwrap()),
            Some(LineType::Range) => ranges.push(get_ranges(line)),
            None => {}
        }
    }

    let mut fresh_ingredients = 0;
    for ingredient_id in ingredient_ids {
        let is_fresh = ranges
            .iter()
            .any(|&(start, end)| ingredient_id >= start && ingredient_id <= end);

        if is_fresh {
            fresh_ingredients += 1;
        }
    }

    println!("{fresh_ingredients}");
}
