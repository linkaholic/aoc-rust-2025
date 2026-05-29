use std::{fs::File, io::Read, io::prelude};

#[cfg(test)]
mod tests;

fn main() {
    let mut f = File::open("./data/input").expect("Error reading file");
    let mut contents = String::new();
    let _ = f.read_to_string(&mut contents);

    forklifts_part1(&contents);
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

    // println!("Final map:");
    // for row in new_map {
    //     print!("\t");
    //     for column in row {
    //         print!("{}", column);
    //     }
    //     println!();
    // }

    let mut new_map_str: Vec<String> = Vec::new();
    for row in new_map {
        let mut row_str = String::new();
        row.iter().for_each(|c| row_str += &(*c).to_string());
        new_map_str.push(row_str);
    }

    return new_map_str;
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

            let block = BlockType::get_block_type(map[m_i][m_j]).unwrap();
            if block == BlockType::PaperRoll {
                adjacent_paper_rolls += 1;
                // println!("found at {}-{}", m_i, m_j);
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

#[derive(PartialEq)]
enum BlockType {
    PaperRoll,
    Nothing,
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
