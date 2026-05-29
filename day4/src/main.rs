use std::{fs::File, io::Read};

#[cfg(test)]
mod tests;

fn main() {
    let mut f = File::open("./data/input").expect("Error reading file");
    let mut contents = String::new();
    let _ = f.read_to_string(&mut contents);

    let removed = forklifts_part2(&contents);
    println!("{}", removed);
}

pub fn forklifts_part2(contents: &String) -> i32 {
    let mut map = string_to_vector(contents);
    let mut aux_map = string_to_vector(contents);
    let mut any_checks_left = true;

    let row_count = map.len();
    let column_count = map[0].len();
    let mut total_paper_rolls_removed = 0;

    while any_checks_left {
        let mut paper_rolls_removed = 0;
        for i in 0..row_count {
            for j in 0..column_count {
                let character = map[i][j];
                let block = BlockType::get_block_type(character).unwrap_or_default();
                if block == BlockType::Nothing {
                    continue;
                }

                aux_map[i][j] = if can_paper_move(&map, i, j) {
                    paper_rolls_removed += 1;
                    'x'
                } else {
                    '@'
                }
            }
        }

        total_paper_rolls_removed += paper_rolls_removed;
        any_checks_left = paper_rolls_removed >= 1;

        update_map(&mut map, &aux_map, row_count, column_count);
        // show_current_map(&map);
    }

    return total_paper_rolls_removed;
}

fn update_map(
    map: &mut Vec<Vec<char>>,
    aux_map: &Vec<Vec<char>>,
    row_count: usize,
    column_count: usize,
) {
    for i in 0..row_count {
        for j in 0..column_count {
            map[i][j] = aux_map[i][j];
        }
    }
}

fn show_current_map(map: &Vec<Vec<char>>) {
    println!("-----------------------");
    for row in map.iter() {
        let mut row_str = String::new();
        row.iter().for_each(|c| row_str += &(*c).to_string());
        println!("{}", row_str);
    }
}

fn can_paper_move(map: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    // println!("---------------------------------------------");
    // println!("checking pos {}-{}", i, j);

    let mut adjacent_paper_rolls = 0;
    let row_count = map.len();
    let column_count = map[0].len();

    let min_i = if i == 0 { 0 } else { i - 1 };
    let max_i = if i == (row_count - 1) {
        row_count - 1
    } else {
        i + 1
    };

    let min_j = if j == 0 { 0 } else { j - 1 };
    let max_j = if j == (column_count - 1) {
        column_count - 1
    } else {
        j + 1
    };

    // println!();

    for m_i in min_i..=max_i {
        for m_j in min_j..=max_j {
            if m_i == i && m_j == j {
                continue;
            }

            // println!("checking {}-{}", m_i, m_j);

            let block = BlockType::get_block_type(map[m_i][m_j]).unwrap_or_default();
            if block == BlockType::PaperRoll {
                adjacent_paper_rolls += 1;
                // println!("found one");
            }

            if adjacent_paper_rolls >= 4 {
                return false;
            }
        }
    }

    return true;
}

fn get_empty_vector(rows: usize, columns: usize) -> Vec<Vec<char>> {
    let mut empty_vector: Vec<Vec<char>> = Vec::new();
    for _ in 0..rows {
        let mut row = Vec::new();
        for _ in 0..columns {
            row.push(' ');
        }
        empty_vector.push(row);
    }

    return empty_vector;
}

fn string_to_vector(contents: &String) -> Vec<Vec<char>> {
    let mut rows: Vec<Vec<char>> = Vec::new();
    for line in contents.lines() {
        let mut row: Vec<char> = Vec::new();
        for character in line.chars() {
            row.push(character);
        }

        rows.push(row);
    }

    return rows;
}

#[derive(PartialEq, Default)]
enum BlockType {
    #[default]
    Nothing,
    PaperRoll,
}

trait BlockTypeCharacter {
    fn get_block_type(c: char) -> Option<BlockType>;
}

impl BlockTypeCharacter for BlockType {
    fn get_block_type(c: char) -> Option<BlockType> {
        match c {
            '@' => Some(BlockType::PaperRoll),
            '.' => Some(BlockType::Nothing),
            _ => None,
        }
    }
}

pub fn forklifts_part1(contents: &String) -> Vec<String> {
    let map = string_to_vector(contents);
    let mut new_map = get_empty_vector(map.len(), map[0].len());
    let mut paper_rolls = 0;

    for (i, row) in map.iter().enumerate() {
        for (j, character) in row.iter().enumerate() {
            let block = BlockType::get_block_type(*character)
                .expect(format!("Unexpected character {}", character).as_str());

            if block == BlockType::Nothing {
                new_map[i][j] = '.';
                continue;
            }

            new_map[i][j] = if can_paper_move(&map, i, j) {
                paper_rolls += 1;
                'x'
            } else {
                '@'
            }
        }
    }

    println!("{}", paper_rolls);

    let mut new_map_str: Vec<String> = Vec::new();
    for row in new_map {
        let mut row_str = String::new();
        row.iter().for_each(|c| row_str += &(*c).to_string());
        println!("{}", row_str);
        new_map_str.push(row_str);
    }

    return new_map_str;
}
