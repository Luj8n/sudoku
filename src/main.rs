use rayon::prelude::*;
use std::time::Instant;

fn can_place(sudoku: &[u8; 81], x: usize, y: usize, digit: u8) -> bool {
  assert!(x < 9 && y < 9 && digit < 10);

  for i in 0..9 {
    let row_i = sudoku[y * 9 + i];
    let collumn_i = sudoku[i * 9 + x];
    let square_i = sudoku[(x / 3) * 3 + i % 3 + (y / 3) * 9 * 3 + (i / 3) * 9];

    if row_i == digit || collumn_i == digit || square_i == digit {
      return false;
    }
  }

  true
}

fn string_to_array(starting_string: &str) -> [u8; 81] {
  assert_eq!(starting_string.len(), 81);
  let mut array: [u8; 81] = [0; 81];
  for (i, char) in starting_string.chars().enumerate() {
    let digit = char.to_digit(10).unwrap() as u8;
    assert!(digit < 10);
    array[i] = digit;
  }
  array
}

fn array_to_string(array: [u8; 81]) -> String {
  let mut string = String::new();
  for y in 0..9 {
    if y % 3 == 0 && y != 0 {
      string += &format!("{}\n", &["-----"; 3].join("+"));
    }
    for x in 0..9 {
      if x % 3 == 0 && x != 0 {
        string += "|";
      }
      string += &array[y * 9 + x].to_string();
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

fn solve_sudoku(sudoku: [u8; 81]) -> [u8; 81] {
  struct RecursionState {
    solved_sudoku: [u8; 81],
  }
  let mut state = RecursionState { solved_sudoku: [0; 81] };

  fn recur(sudoku: [u8; 81], state: &mut RecursionState) -> bool {
    for y in 0..9 {
      for x in 0..9 {
        if sudoku[y * 9 + x] == 0 {
          for digit in 1..=9 {
            if !can_place(&sudoku, x, y, digit) {
              continue;
            }

            let mut next_sudoku = sudoku;
            next_sudoku[y * 9 + x] = digit;

            if recur(next_sudoku, state) {
              return true;
            }
          }
          return false;
        }
      }
    }

    state.solved_sudoku = sudoku;

    true
  }

  recur(sudoku, &mut state);

  state.solved_sudoku
}

fn main() {
  let file_name = std::env::args().nth(1).expect("Please provide the file name");

  let file_content = std::fs::read_to_string(&file_name).unwrap_or_else(|_| panic!("'{}' not found", file_name));

  let sudokus: Vec<[u8; 81]> = file_content
    .lines()
    .map(|line| string_to_array(line.split_whitespace().nth(1).unwrap()))
    .take(50000)
    .collect();

  let current_time = Instant::now();

  let _solutions = sudokus
    .par_iter()
    .map(|sudoku| solve_sudoku(*sudoku))
    .collect::<Vec<[u8; 81]>>();

  let duration = current_time.elapsed();

  // for solution in solutions {
  //   println!("{}\n", array_to_string(solution));
  // }

  println!("{:?}", duration);
}
