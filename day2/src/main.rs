use std::{fs::File, io::Read};

fn main() {
    let mut f = File::open("./data/input").expect("Error reading file");
    let mut content = String::new();
    let _ = f.read_to_string(&mut content);

    let mut invalid_ids = 0;
    for line in content.lines() {
        let ranges = line.split(",");

        for range in ranges {
            let (start, end) = get_word_range(range);

            for current_value in start..=end {
                let aux_word = current_value.to_string();
                let word = aux_word.as_str();
                let word_len = word.len();

                if word_len == 0 {
                    continue;
                }

                let mut checks: Vec<usize> = Vec::new();
                get_word_checks(word_len, &mut checks);

                for check in checks {
                    if has_invalid_id(word, check) {
                        invalid_ids += current_value;
                        break;
                    }
                }
            }
        }
    }

    println!("{invalid_ids}");
}

fn has_invalid_id(word: &str, check: usize) -> bool {
    let word_len = word.len();
    let mut are_equal = true;
    let first_split_word = &word[0..check];

    for it in (check..word_len).step_by(check) {
        let current_split_word = &word[it..(check + it)];
        if first_split_word != current_split_word {
            are_equal = false;
            break;
        }
    }

    are_equal
}

fn get_word_checks(word_len: usize, checks: &mut Vec<usize>) {
    for j in (1..=word_len - 1).rev() {
        if (word_len % j) == 0 {
            checks.push(j);
        }
    }
}

fn get_word_range(current_range: &str) -> (i64, i64) {
    let range = current_range.trim_matches(' ');

    let mut split_range = range.split("-");
    let start_str = split_range.next().unwrap();
    let end_str = split_range.next().unwrap();

    let start = start_str.parse::<i64>().unwrap();
    let end = end_str.parse::<i64>().unwrap();

    return (start, end);
}
