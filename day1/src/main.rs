use std::fs;

fn main() {
    let content = fs::read_to_string("./data/input").unwrap();
    let mut dial = 50;
    let mut iterations = 0;

    let lines = content.split("\n");
    for line in lines {
        let mut chars = line.chars();
        let direction = chars.next().unwrap_or_default();
        let str_rotations = chars.as_str();
        let rotations = str_rotations.parse::<i32>().unwrap_or_default();

        if !(direction == 'L' || direction == 'R') {
            continue;
        }

        for _ in 0..rotations {
            if dial == 99 && direction == 'R' {
                dial = 0;
            } else if direction == 'R' {
                dial += 1;
            }

            if dial == 0 && direction == 'L' {
                dial = 99;
            } else if direction == 'L' {
                dial -= 1;
            }

            // Part two
            // if dial == 0 {
            //     iterations += 1;
            // }
        }

        // Part one
        if dial == 0 {
            iterations += 1;
        }
    }

    println!("The answer is {}", iterations);
}
