use std::{fs::File, io::Read, str::from_utf8, thread::current};

fn main() {
    let mut f = File::open("./data/sample").expect("Error reading file");
    let mut content = String::new();
    let _ = f.read_to_string(&mut content);

    let mut invalid_ids = 0;
    for line in content.lines() {
        let ranges = line.split(",");
        for mut range in ranges {
            range = range.trim_matches(' ');

            let mut split_range = range.split("-");
            let start_str = split_range.next().unwrap();
            let end_str = split_range.next().unwrap();

            let start = start_str.parse::<i64>().unwrap();
            let end = end_str.parse::<i64>().unwrap();

            println!("Now checking: {start} - {end}...");
            for i in start..=end {
                let current_word = i.to_string();
                const CURRENT_LEN: usize = current_word.len();

                for j in (0..CURRENT_LEN).rev() {
                    if (j % CURRENT_LEN) != 0 {
                        continue;
                    }

                    println!("checking against {current_word}");

                    let mut words = [""; CURRENT_LEN];
                    let mut full_word = current_word.as_str();
                    for k in j..CURRENT_LEN {
                        let (word, aux_word) = full_word.split_at(j);
                        full_word = aux_word;
                        words[k] = word;
                        println!("{word}");
                    }

                    println!("split words;");
                    for word in words {
                        println!("\t{word}");
                    }

                    // if are_equal {
                    //     println!("valid");
                    //     invalid_ids += i;
                    //     break;
                    // }
                }
            }
        }
    }

    println!("{invalid_ids}");
}
