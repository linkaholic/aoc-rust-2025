use std::fs;

fn main() {
    let content =
        fs::read_to_string("./data/input").expect("An error occurred reading the input puzzle");
    let ranges = content.split(",");
    let mut sum_of_ids = 0;

    for range in ranges {
        let mut current_range = range.split("-");
        let first_range = current_range.next();
        let second_range = current_range.next();
        if first_range == None || second_range == None {
            continue;
        }

        let x = match first_range.unwrap().parse::<i64>() {
            Ok(number) => number,
            Err(_) => continue,
        };

        let y = match second_range.unwrap().parse::<i64>() {
            Ok(number) => number,
            Err(_) => continue,
        };

        for n in x..y {
            if n.to_string().len() % 2 != 0 {
                continue;
            }

            let n_str = n.to_string();
            let str_size = n_str.len() / 2;
            let split_str = n_str.split_at(str_size);
            if split_str.0 != split_str.1 {
                continue;
            }

            sum_of_ids += n;
        }
    }

    println!("The answer is {}", sum_of_ids);
}
