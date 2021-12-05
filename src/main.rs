use std::fmt::Display;
use std::io::BufReader;
use std::fs::File;
use std::fmt::Formatter;
use std::fmt::Error;
use std::io::Read;
use std::collections::HashSet;

struct SudokuBoard {
    contents: [[Option<u32>; 9]; 9]
}

impl SudokuBoard {
    fn new() -> SudokuBoard {
        SudokuBoard {
            contents: [
                [None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None, None],
            ]
        }
    }

    fn from_file(path: &str) -> SudokuBoard {
        let mut board = SudokuBoard::new();

        let mut reader = BufReader::new(File::open(path).unwrap());
        let mut contents = String::with_capacity(340);
        reader.read_to_string(&mut contents).unwrap();

        // Search for numbers and periods
        let mut row = 0;
        let mut col = 0;

        for c in contents.chars() {
            match c {
                _ if c.is_ascii_digit() || c=='.' => {
                    board.contents[row][col] = c.to_digit(10);

                    if col==8 {
                        row+=1;
                        col=0;
                    } else {
                        col +=1;
                    }
                }
                _ => {}
            }
        }

        board
    }

    fn is_full(&self) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if self.contents[row][col].is_none() {
                    return false;
                }
            }
        }

        return true
    }

    fn solve(&mut self) -> bool {
        if self.is_full() {
            return self.check();
        }

        // Board has empty squares
        let mut row = 0;
        let mut col = 0;

        // Find the next available space
        while self.contents[row][col].is_some() {
            if col==8{
                row+=1;
                col=0;
            } else {
                col+=1;
            }
        }

        // Set it to the every possible value
        for test_val in 1..10 {
            self.contents[row][col] = Some(test_val);

            // If it fails the check go on to the next one
            if !self.check() {
                continue;
            }

            if self.solve() {
                return true;
            }
        }

        self.contents[row][col] = None;

        return false;
    }

    fn check(&self) -> bool {
        for index in 0..9 {
            if !self.check_row(index) || !self.check_col(index) || !self.check_group(index) {
                return false;
            }
        }

        return true;
    }

    fn check_row(&self, id: usize) -> bool {
        let mut set = HashSet::new();

        for col in 0..9 {
            let val = self.contents[id][col];
            if val.is_none() {
                continue;
            }

            let val = val.unwrap();

            if set.contains(&val) {
                return false;
            }

            set.insert(val);
        }

        return true;
    }

    fn check_col(&self, id: usize) -> bool {
        let mut set = HashSet::new();

        for row in 0..9 {
            let val = self.contents[row][id];
            if val.is_none() {
                continue;
            }

            let val = val.unwrap();

            if set.contains(&val) {
                return false;
            }

            set.insert(val);
        }

        return true;
    }

    fn check_group(&self, id: usize) -> bool {
        let mut set = HashSet::new();

        let row_start = (id / 3)*3;
        let col_start = (id % 3)*3;

        for row in row_start..row_start+3 {
            for col in col_start..col_start+3 {
                let val = self.contents[row][col];
                if val.is_none() {
                    continue;
                }

                if set.contains(&val) {
                    return false;
                }

                set.insert(val);
            }
        }

        return true;
    }
}

fn num_fmt(num: Option<u32>) -> String {
    match num {
        Some(x) => x.to_string(),
        None => '.'.to_string()
    }
}

impl Display for SudokuBoard {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        fmt.write_str("+-------+-------+-------+\n")?;

        for group in 0..3 {
            for row in 0..3 {
                let row_i = 3*group+row;
                fmt.write_str(
                    &format!("| {} {} {} | {} {} {} | {} {} {} |\n",
                    num_fmt(self.contents[row_i][0]),
                    num_fmt(self.contents[row_i][1]),
                    num_fmt(self.contents[row_i][2]),
                    num_fmt(self.contents[row_i][3]),
                    num_fmt(self.contents[row_i][4]),
                    num_fmt(self.contents[row_i][5]),
                    num_fmt(self.contents[row_i][6]),
                    num_fmt(self.contents[row_i][7]),
                    num_fmt(self.contents[row_i][8]),
                ))?;
            }
            fmt.write_str("+-------+-------+-------+\n")?;
        }
        Ok(())
    }
}

fn main() {
    let mut board = SudokuBoard::from_file("./sudoku.txt");
    board.solve();
    println!("{}", board);
}
