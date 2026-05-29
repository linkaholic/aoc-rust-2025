use std::{
    fmt::{self, Display},
    fs::File,
    io::Read,
};

fn main() {
    let mut f = File::open("./data/input").expect("Error reading the file");
    let mut content = String::new();
    let _ = f.read_to_string(&mut content);

    math_homework_part1(content);
}

fn math_homework_part1(content: String) {
    let homework = string_to_vector(&content);

    let mut results: Vec<(Operation, i64)> = Vec::new();

    let end = homework[0].len();
    let start = homework.len();

    for j in 0..end {
        for i in (0..start).rev() {
            let word = homework[i][j];
            let line_option = results.get_mut(j);

            if line_option.is_none() {
                let column_operation =
                    Operation::get_operation_type(word).expect("Error reading operation");

                let default_value = if column_operation == Operation::Multiplication {
                    1
                } else {
                    0
                };

                results.push((column_operation, default_value));
            } else {
                let (operation, result) = line_option.unwrap();
                let value = homework[i][j].parse::<i64>().unwrap();

                match operation {
                    Operation::Sum => *result += value,
                    Operation::Multiplication => *result *= value,
                }
            }
        }
    }

    let total_sum = results.iter().map(|(_, res)| res).sum::<i64>();
    println!("{}", total_sum);
}

fn string_to_vector(contents: &String) -> Vec<Vec<&str>> {
    let mut rows: Vec<Vec<&str>> = Vec::new();
    for line in contents.lines() {
        let mut row: Vec<&str> = Vec::new();
        for word in line.split_whitespace() {
            row.push(word);
        }

        rows.push(row);
    }

    return rows;
}

#[derive(PartialEq)]
enum Operation {
    Sum,
    Multiplication,
}

trait OperationTypeCharacter {
    fn get_operation_type(operation: &str) -> Option<Operation>;
}

impl OperationTypeCharacter for Operation {
    fn get_operation_type(operation: &str) -> Option<Operation> {
        match operation {
            "+" => Some(Operation::Sum),
            "*" => Some(Operation::Multiplication),
            _ => None,
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Operation::Sum => write!(f, "+"),
            Operation::Multiplication => write!(f, "*"),
        }
    }
}
