use std::io::{self, Write};

use colored::*;
use regex::Regex;

pub const SIZE: usize = 9;

#[derive(Debug)]
pub struct Sudoku {
    pub field: [[u8; SIZE]; SIZE],
    pub option: [[Vec<u8>; SIZE]; SIZE],
}

impl Default for Sudoku {
    fn default() -> Self {
        Self::new()
    }
}

impl Sudoku {
    pub fn new() -> Self {
        Self {
            field: [[0; SIZE]; SIZE],
            option: Default::default(),
        }
    }

    pub fn print(&self) {
        println!("{}", "  | 0  1  2 | 3  4  5 | 6  7  8 |".green());
        println!("{}", "--+---------+---------+---------+".green());
        for y in 0..SIZE {
            print!("{} {}", y.to_string().green(), "|".green());
            for x in 0..SIZE {
                let value = self.field[y][x];
                if value == 0 {
                    print!("   ");
                } else {
                    print!(" {} ", value);
                }
                if (x + 1) % 3 == 0 {
                    print!("{}", "|".green());
                }
            }
            println!();
            if (y + 1) % 3 == 0 {
                println!("{}", "--+---------+---------+---------+".green());
            }
        }
    }

    #[allow(dead_code)]
    pub fn print_option(&self) {
        for y in 0..SIZE {
            println!("{:?}", self.option[y]);
        }
    }

    #[allow(dead_code)]
    pub fn man_init(&mut self) {
        let re = Regex::new(r"^[1-9]$").unwrap();
        println!("Input 0 for none or 1~9");

        for y in 0..SIZE {
            for x in 0..SIZE {
                loop {
                    print!("({},{}) :", y, x);
                    io::stdout().flush().unwrap();

                    let mut input = String::new();
                    io::stdin().read_line(&mut input).unwrap();
                    let input = input.trim();

                    if re.is_match(input) {
                        self.field[y][x] = input.parse().unwrap();
                        break;
                    } else {
                        println!("Invalid input.")
                    }
                }
            }
            println!();
        }
    }

    pub fn static_init(&mut self) {
        // self.field = [
        //     [0, 8, 0, 0, 0, 1, 0, 0, 0],
        //     [0, 7, 1, 6, 0, 0, 0, 0, 0],
        //     [6, 4, 0, 2, 0, 0, 0, 9, 0],
        //     [0, 0, 0, 0, 0, 0, 0, 0, 4],
        //     [0, 0, 0, 1, 5, 0, 0, 0, 0],
        //     [0, 9, 0, 4, 6, 0, 0, 0, 0],
        //     [0, 0, 3, 0, 0, 2, 0, 4, 6],
        //     [0, 0, 2, 0, 7, 5, 0, 0, 0],
        //     [0, 0, 0, 8, 0, 0, 0, 3, 0],
        // ];
        // self.field = [
        //     [0, 9, 0, 0, 0, 3, 0, 0, 0],
        //     [2, 0, 0, 0, 0, 9, 1, 0, 0],
        //     [0, 0, 0, 0, 0, 4, 0, 0, 8],
        //     [0, 0, 7, 0, 0, 0, 9, 0, 0],
        //     [0, 8, 4, 0, 1, 0, 0, 0, 0],
        //     [0, 0, 0, 3, 0, 0, 0, 4, 0],
        //     [4, 5, 0, 0, 2, 0, 0, 6, 0],
        //     [0, 1, 0, 0, 5, 0, 0, 0, 0],
        //     [0, 0, 0, 8, 0, 0, 7, 0, 4],
        // ];
        self.field = [
            [0, 0, 0, 0, 6, 0, 5, 0, 0],
            [0, 0, 2, 0, 0, 0, 0, 0, 4],
            [0, 1, 0, 3, 0, 0, 0, 9, 0],
            [0, 3, 4, 5, 0, 0, 0, 0, 6],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [2, 0, 0, 0, 0, 9, 8, 1, 0],
            [0, 5, 0, 0, 0, 8, 0, 3, 0],
            [3, 0, 0, 0, 0, 0, 9, 0, 0],
            [0, 0, 6, 0, 1, 0, 0, 0, 0],
        ];
    }

    pub fn solve(&mut self) {
        self.update_option();
        loop {
            self.print();
            println!();

            self.sole_candidate_rule();
            if self.update_option() {
                break;
            }
            self.row_reduction();
            if self.update_option() {
                break;
            }
            self.column_reduction();
            if self.update_option() {
                break;
            }
            self.box_reduction();
            if self.update_option() {
                break;
            }
        }
        self.print();
    }

    pub fn sole_candidate_rule(&mut self) {
        println!("sole_candidate_rule");
        for y in 0..SIZE {
            for x in 0..SIZE {
                if self.option[y][x].len() == 1 {
                    self.determine(y, x, self.option[y][x][0]);
                }
            }
        }
    }

    pub fn row_reduction(&mut self) {
        println!("row_reduction");
        for y in 0..SIZE {
            for x in 0..SIZE {
                let row_options = self.row_option(y);
                for option in self.option[y][x].iter() {
                    let count = row_options.iter().filter(|&i| *i == *option).count();
                    if count == 1 {
                        self.determine(y, x, *option);
                        break;
                    }
                }
            }
        }
    }

    pub fn column_reduction(&mut self) {
        println!("column_reduction");
        for y in 0..SIZE {
            for x in 0..SIZE {
                let column_options = self.column_option(x);
                for option in self.option[y][x].iter() {
                    let count = column_options.iter().filter(|&i| *i == *option).count();
                    if count == 1 {
                        self.determine(y, x, *option);
                        break;
                    }
                }
            }
        }
    }

    pub fn box_reduction(&mut self) {
        println!("box_reduction");
        for y in 0..SIZE {
            for x in 0..SIZE {
                let subgrid_options = self.subgrid_option(y, x);
                for option in self.option[y][x].iter() {
                    let count = subgrid_options.iter().filter(|&i| *i == *option).count();
                    if count == 1 {
                        self.determine(y, x, *option);
                        break;
                    }
                }
            }
        }
    }

    pub fn update_option(&mut self) -> bool {
        let mut counter = 0;
        for y in 0..SIZE {
            for x in 0..SIZE {
                self.option[y][x].clear();
                if self.field[y][x] != 0 {
                    counter += 1;
                    continue;
                }
                for i in 1..=SIZE {
                    if !self.row(y).contains(&(i as u8))
                        && !self.column(x).contains(&(i as u8))
                        && !self.subgrid(y, x).contains(&(i as u8))
                    {
                        self.option[y][x].push(i as u8);
                    }
                }
            }
        }
        counter == SIZE * SIZE
    }

    pub fn determine(&mut self, y: usize, x: usize, n: u8) {
        println!("({}, {}):{}  candidates:{:?}", y, x, n, self.option[y][x]);
        self.field[y][x] = n;
        self.option[y][x].clear();
    }

    pub fn row(&self, y: usize) -> Vec<u8> {
        let mut options = Vec::<u8>::new();
        for x in 0..SIZE {
            if self.field[y][x] != 0 {
                options.push(self.field[y][x]);
            }
        }
        options
    }

    pub fn column(&self, x: usize) -> Vec<u8> {
        let mut options = Vec::<u8>::new();
        for y in 0..SIZE {
            if self.field[y][x] != 0 {
                options.push(self.field[y][x]);
            }
        }
        options
    }

    pub fn subgrid(&self, y: usize, x: usize) -> Vec<u8> {
        let mut options = Vec::<u8>::new();
        let i = (y / 3) * 3;
        let j = (x / 3) * 3;
        for y in 0..3 {
            for x in 0..3 {
                if self.field[i + y][j + x] != 0 {
                    options.push(self.field[i + y][j + x]);
                }
            }
        }
        options
    }

    pub fn row_option(&self, y: usize) -> Vec<u8> {
        let mut options = Vec::<u8>::new();
        for x in 0..SIZE {
            if self.field[y][x] == 0 {
                options.extend(&self.option[y][x]);
            }
        }
        options
    }

    pub fn column_option(&self, x: usize) -> Vec<u8> {
        let mut options = Vec::<u8>::new();
        for y in 0..SIZE {
            if self.field[y][x] == 0 {
                options.extend(&self.option[y][x]);
            }
        }
        options
    }

    pub fn subgrid_option(&self, y: usize, x: usize) -> Vec<u8> {
        let mut options = Vec::<u8>::new();
        let i = (y / 3) * 3;
        let j = (x / 3) * 3;
        for y in 0..3 {
            for x in 0..3 {
                if self.field[i + y][j + x] == 0 {
                    options.extend(&self.option[i + y][j + x]);
                }
            }
        }
        options
    }
}
