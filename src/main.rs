use itertools::Itertools;
use rayon::prelude::*;
struct SudokuGrid {
  array: [u8; 81],
}

impl SudokuGrid {
  fn new(starting_string: &str) -> SudokuGrid {
    assert_eq!(starting_string.len(), 81);
    let mut array: [u8; 81] = [0; 81];
    for (i, char) in starting_string.chars().enumerate() {
      let digit = char.to_digit(10).unwrap() as u8;
      array[i] = digit;
    }
    SudokuGrid { array }
  }
}

// impl ToString for SudokuGrid {
//   fn to_string(&self) -> String {
//     self.array.chunks_exact(9).map(|row| row.join("|"));
//     "".to_string()
//   }
// }

fn main() {
  let lines: Vec<SudokuGrid> = std::fs::read_to_string("easy.txt")
    .expect("easy.txt not found")
    .par_lines()
    .map(|line| SudokuGrid::new(line.split_whitespace().nth(1).unwrap()))
    .collect();

  // println!("'{}'", lines.get(1).unwrap().grid);
}
