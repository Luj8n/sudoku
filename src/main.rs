use rayon::prelude::*;
use std::time::{Duration, Instant};

#[derive(Clone, PartialEq)]
struct Sudoku {
  array: [u8; 81],
}

impl Sudoku {
  fn new(starting_array: [u8; 81]) -> Self {
    Sudoku { array: starting_array }
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

impl Default for Sudoku {
  fn default() -> Self {
    Self { array: [0; 81] }
  }
}

impl ToString for Sudoku {
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

struct SudokuSolution {
  initial: Sudoku,
  solution: Sudoku,
  duration: Duration,
}

fn solve_sudoku(sudoku: &Sudoku) -> SudokuSolution {
  struct RecursionState {
    solved_sudoku: Sudoku,
  }

  let mut state = RecursionState {
    solved_sudoku: Sudoku::default(),
  };

  fn recur(sudoku: Sudoku, state: &mut RecursionState) -> bool {
    // println!("\n{}", sudoku.to_string());

    let mut zero_count = 0;

    for y in 0..9 {
      for x in 0..9 {
        if *sudoku.get(x, y) == 0 {
          zero_count += 1;
        }
      }
    }

    if zero_count == 0 {
      state.solved_sudoku = sudoku;
      return true;
    }

    for y in 0..9 {
      for x in 0..9 {
        if *sudoku.get(x, y) == 0 {
          for digit in 1..=9 {
            if !sudoku.can_place(x, y, digit) {
              continue;
            }

            let mut next_sudoku = sudoku.clone();
            next_sudoku.change(x, y, digit);

            if recur(next_sudoku, state) {
              return true;
            }
          }
          return false;
        }
      }
    }

    false
  }

  let current_time = Instant::now();

  let solved = recur(sudoku.clone(), &mut state);

  let duration = current_time.elapsed();

  SudokuSolution {
    initial: sudoku.clone(),
    solution: state.solved_sudoku,
    duration,
  }
}

fn main() {
  let file_name = "diabolical.txt";

  let file_content = std::fs::read_to_string(file_name).unwrap_or_else(|_| panic!("'{}' not found", file_name));

  let sudokus: Vec<Sudoku> = file_content
    .lines()
    .map(|line| Sudoku::from_str(line.split_whitespace().nth(1).unwrap()))
    .take(10000)
    .collect();

  let current_time = Instant::now();

  let solutions = sudokus
    .par_iter()
    .map(|sudoku| solve_sudoku(sudoku))
    .collect::<Vec<SudokuSolution>>();

  let duration = current_time.elapsed();

  println!("{:?}", duration);
}
