use std::fs;
use rand::Rng;

const SIZE: usize = 29;
const STARGAUGE_PATH: &str = "/Users/derekzhu/Code/Quant/stargauge/src/stargauge.txt";

const STANZAS: usize = 4;

pub struct Range {
    pub start: (usize, usize),
    pub end: (usize, usize),
}

// const RED_RANGES: [Range; 8] = [
//     // Rows
//     Range { start: (0, 0), end: (0, 28) },
//     Range { start: (28, 0), end: (28, 28) },
//     Range { start: (7, 0), end: (7, 28) },
//     Range { start: (21, 0), end: (21, 28) },

//     // Columns
//     Range { start: (0, 0), end: (28, 0) },
//     Range { start: (0, 7), end: (28, 7) },
//     Range { start: (0, 21), end: (28, 21) },
//     Range { start: (0, 28), end: (28, 28) },
// ];

// const BLACK_RANGES: [Range; 4] = [
//     Range { start: (1, 1), end: (6, 6) },
//     Range { start: (1, 22), end: (6, 27) },
//     Range { start: (22, 1), end: (27, 6) },
//     Range { start: (22, 22), end: (27, 27) },
// ];

fn main() {
    let mut matrix: [[char; SIZE]; SIZE] = [[' '; SIZE]; SIZE];

    let _ = load_stargauge(&mut matrix, STARGAUGE_PATH);
    println!("File content:\n {}", matrix[0][0]);

    let (red_chars, red_indices) = red_pattern(&matrix);
    for i in 0..red_chars.len() {
        println!("Red Stanza {}: {:?} at {:?}", i + 1, red_chars[i], red_indices[i]);
    }
    // print indices
    for i in 0..red_indices.len() {
        println!("Red Stanza {} Index: {:?}", i + 1, red_indices[i]);
    }

    println!("Red Stanzas as String:\n{}", to_string(&red_chars, 1));
}

pub fn load_stargauge(matrix: &mut [[char; SIZE]; SIZE], file_path: &str) -> std::io::Result<()> {
    let content = fs::read_to_string(file_path)?;
    for (i, line) in content.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if i < SIZE && j < SIZE {
                matrix[i][j] = ch;
            }
        }
    }

    Ok(())
}

pub fn to_string(vec: &[Vec<char>; STANZAS], format_op: u8) -> String {
    let mut s = String::new();
    for (i, stanza) in vec.iter().enumerate() {
        s.push_str(stanza.iter().collect::<String>().as_str());
        match format_op {
            0 => s.push('\n'),
            1 => s.push_str(if i < vec.len() - 1 { ", " } else { "" }),
            _ => (),
        }
    }
    s
}

pub fn red_pattern(matrix: &[[char; SIZE]; SIZE]) -> ([Vec<char>; 4], [(usize, usize); 4]) {
    let mut rng = rand::rng();

    let mut row_i = rng.random_range(0..5) * 7;
    let mut col_i = rng.random_range(0..5) * 7;

    if col_i == 14 && row_i == 14 {
        let r_offset = if rng.random_range(0..2) == 1 { 1 } else { -1 };
        let c_offset = if rng.random_range(0..2) == 1 { 1 } else { -1 };    
        let r_step = rng.random_range(1..3) as isize;
        let c_step = rng.random_range(1..3) as isize;

        row_i = (row_i as isize + r_offset * r_step) as usize;
        col_i = (col_i as isize + c_offset * c_step) as usize;
    }

    let mut chars: [Vec<char>; STANZAS] = [(); STANZAS].map(|_| Vec::new());
    let mut indices: [(usize, usize); STANZAS] = [(0, 0); STANZAS];


    for i in 0..STANZAS {
        indices[i] = (row_i, col_i);
        let mut directions = Vec::new();

        if row_i >= 7 { directions.push((-1, 0)); } 
        if row_i + 7 < SIZE { directions.push((1, 0)); }
        if col_i >= 7 { directions.push((0, -1)); } 
        if col_i + 7 < SIZE { directions.push((0, 1)); }

        // Prevent reversing direction (going back to previous stanza)
        // TODO: Refactor to get unique lines every time
        if i > 0 {
            let (pr, pc) = indices[i-1];
            if col_i + 7 == pc {
                directions.retain(|&x| x != (0, 1));
            } else if col_i.saturating_sub(7) == pc {
                directions.retain(|&x| x != (0, -1));
            } else if row_i + 7 == pr {
                directions.retain(|&x| x != (1, 0));
            } else if row_i.saturating_sub(7) == pr {
                directions.retain(|&x| x != (-1, 0));
            }
        }

        // Prevent moving into the center (14, 14) if it's not the current position
        if col_i + 7 == 14 && row_i == 14 {
            directions.retain(|&x| x != (0, 1));
        } else if col_i.saturating_sub(7) == 14 && row_i == 14 {
            directions.retain(|&x| x != (0, -1));
        } else if row_i + 7 == 14 && col_i == 14 {
            directions.retain(|&x| x != (1, 0));
        } else if row_i.saturating_sub(7) == 14 && col_i == 14 {
            directions.retain(|&x| x != (-1, 0));
        }
        
        if !directions.is_empty() {
            let (dr, dc) = directions[rng.random_range(0..directions.len())];
            for _ in 0..7 {
                println!("row: {}, col: {}", row_i, col_i);
                chars[i].push(matrix[row_i][col_i]);
                row_i = (row_i as isize + dr) as usize;
                col_i = (col_i as isize + dc) as usize;
            }
        }
    }


    (chars, indices)
}
