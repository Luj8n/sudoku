use itertools::Itertools;
use rayon::prelude::*;

#[derive(Clone, PartialEq)]
struct SudokuGrid {
  array: [u8; 81],
}

impl SudokuGrid {
  fn new(starting_array: [u8; 81]) -> Self {
    SudokuGrid { array: starting_array }
  }

  fn from_str(starting_string: &str) -> Self {
    assert_eq!(starting_string.len(), 81);
    let mut array: [u8; 81] = [0; 81];
    for (i, char) in starting_string.chars().enumerate() {
      let digit = char.to_digit(10).unwrap() as u8;
      assert!(digit < 10);
      array[i] = digit;
    }
    Self { array }
  }

  fn get(&self, x: usize, y: usize) -> &u8 {
    assert!(x < 9 && y < 9);
    &self.array[y * 9 + x]
  }

  fn change(&mut self, x: usize, y: usize, new_value: u8) {
    assert!(x < 9 && y < 9 && new_value < 10);
    self.array[y * 9 + x] = new_value;
  }

  fn can_place(&self, x: usize, y: usize, digit: u8) -> bool {
    assert!(x < 9 && y < 9 && digit < 10);

    for i in 0..9 {
      let row_i = self.array[y * 9 + i];
      let collumn_i = self.array[i * 9 + x];
      let square_i = self.array[(x / 3) * 3 + i % 3 + (y / 3) * 9 * 3 + (i / 3) * 9];

      if row_i == digit || collumn_i == digit || square_i == digit {
        return false;
      }
    }

    true
  }
}

impl Default for SudokuGrid {
  fn default() -> Self {
    Self { array: [0; 81] }
  }
}

impl ToString for SudokuGrid {
  fn to_string(&self) -> String {
    let mut string = String::new();
    for y in 0..9 {
      if y % 3 == 0 && y != 0 {
        string += &format!("{}\n", &["-----"; 3].join("+"));
      }
      for x in 0..9 {
        if x % 3 == 0 && x != 0 {
          string += "|";
        }
        string += &self.array[y * 9 + x].to_string();
        if x % 3 < 2 {
          string += " ";
        }
      }
      if y != 8 {
        string += "\n";
      }
    }
    string
  }
}

fn main() {
  let file = "easy.txt";

  let sudokus: Vec<SudokuGrid> = std::fs::read_to_string(file)
    .expect(&format!("'{}' not found", file))
    .par_lines()
    .map(|line| SudokuGrid::from_str(line.split_whitespace().nth(1).unwrap()))
    .collect();

  struct RecursionState {
    solved: bool,
    solved_sudoku: SudokuGrid,
    all_sudokus: Vec<SudokuGrid>,
  }

  let sudoku = sudokus.get(0).unwrap().to_owned();

  let mut state = RecursionState {
    solved: false,
    solved_sudoku: SudokuGrid::default(),
    all_sudokus: vec![sudoku.clone()],
  };

  fn recur(sudoku: SudokuGrid, state: &mut RecursionState) {
    if state.solved {
      return;
    }

    // println!("\n{}", sudoku.to_string());

    let mut no_zeros = true;

    for y in 0..9 {
      for x in 0..9 {
        if *sudoku.get(x, y) == 0 {
          no_zeros = false;

          for digit in 1..=9 {
            let mut new_sudoku = sudoku.clone();
            new_sudoku.change(x, y, digit);

            if state.all_sudokus.par_iter().any(|s| *s == new_sudoku) {
              continue;
            }

            state.all_sudokus.push(new_sudoku.clone());

            if state.all_sudokus.len() % 1000 == 0 {
              println!("\n{}", state.all_sudokus.len());
            }

            if sudoku.can_place(x, y, digit) {
              recur(new_sudoku, state);
            }
          }
        }
      }
    }

    if no_zeros {
      state.solved = true;
      state.solved_sudoku = sudoku;
    }
  }

  println!("Initial sudoku:\n{}", sudoku.to_string());

  recur(sudoku, &mut state);

  println!("\nSolved sudoku:\n{}", state.solved_sudoku.to_string());
}
