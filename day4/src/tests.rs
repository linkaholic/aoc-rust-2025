use std::{collections::HashMap, fs::File, io::Read};

use super::*;

#[test]
fn test_forklifts() {
    use crate::forklifts_part1;

    let contents = read_content("sample");
    let mut results: HashMap<usize, &str> = HashMap::new();
    results.insert(0, "..xx.xx@x.");
    results.insert(1, "x@@.@.@.@@");
    results.insert(2, "@@@@@.x.@@");
    results.insert(3, "@.@@@@..@.");
    results.insert(4, "x@.@@@@.@x");
    results.insert(5, ".@@@@@@@.@");
    results.insert(6, ".@.@.@.@@@");
    results.insert(7, "x.@@@.@@@@");
    results.insert(8, ".@@@@@@@@.");
    results.insert(9, "x.x.@@@.x.");

    let new_map = forklifts_part1(&contents);

    for (i, row) in new_map.iter().enumerate() {
        assert_eq!(row, results.remove(&i).unwrap());
    }
}

fn read_content(file_name: &str) -> String {
    let path = format!("./data/{}", file_name);
    let mut f = File::open(path).expect("Error reading file");
    let mut content = String::new();
    let _ = f.read_to_string(&mut content);
    content
}
