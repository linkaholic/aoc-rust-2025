use std::collections::HashMap;

use super::*;

#[test]
fn joltage_calculator() {
    let content = get_file_content("sample");

    let mut test_results: HashMap<usize, i64> = HashMap::new();
    test_results.insert(0, 987654321111);
    test_results.insert(1, 811111111119);
    test_results.insert(2, 434234234278);
    test_results.insert(3, 888911112111);

    let mut total_joltage = 0;
    for (i, line) in content.lines().enumerate() {
        let current_joltage = get_largest_combination_part2(line, 12);
        assert_eq!(current_joltage, *(test_results.get(&i).unwrap()));
        total_joltage += current_joltage;
    }

    assert_eq!(total_joltage, 3121910778619);
}

#[test]
fn small_sample() {
    let content = get_file_content("smaller");
    for line in content.lines() {
        let res = get_largest_combination_part2(content.as_str(), 4);
        assert_eq!(res, 9988);
    }
}

fn get_file_content(file_name: &str) -> String {
    let path = format!("./data/{}", file_name);
    let mut f = File::open(path).expect("Error reading file");
    let mut content = String::new();
    let _ = f.read_to_string(&mut content);
    content
}
