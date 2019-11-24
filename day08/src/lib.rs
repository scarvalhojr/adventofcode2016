use regex::Regex;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::io::{Error, ErrorKind};
use std::str::FromStr;
use Instruction::*;

pub struct Screen {
    num_cols: usize,
    num_rows: usize,
    lit_pixels: HashSet<(usize, usize)>,
}

pub enum Instruction {
    Rectangle(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}

impl Screen {
    pub fn new(num_cols: usize, num_rows: usize) -> Self {
        Self {
            num_cols,
            num_rows,
            lit_pixels: HashSet::new(),
        }
    }

    pub fn count_lit_pixels(&self) -> usize {
        self.lit_pixels.len()
    }

    pub fn execute(&mut self, instructions: &[Instruction]) {
        instructions.iter().for_each(|instr| {
            match *instr {
                Rectangle(cols, rows) => self.draw_rectanle(cols, rows),
                RotateRow(row, rotation) => self.rotate_row(row, rotation),
                RotateColumn(col, rotation) => self.rotate_col(col, rotation),
            }
        });
    }

    fn draw_rectanle(&mut self, cols: usize, rows: usize) {
        if cols < self.num_cols && rows < self.num_rows {
            self.lit_pixels.extend((0..cols).map(|c| (c, 0)));
            self.lit_pixels.extend((0..cols).map(|c| (c, rows - 1)));
            self.lit_pixels.extend((1..rows - 1).map(|r| (0, r)));
            self.lit_pixels.extend((1..rows - 1).map(|r| (cols - 1, r)));
        }
    }

    fn rotate_row(&mut self, row: usize, rotation: usize) {
        let lit_cols = self
            .lit_pixels
            .iter()
            .filter(|(_, r)| *r == row)
            .map(|(c, _)| *c)
            .collect::<Vec<_>>();
        for col in &lit_cols {
            self.lit_pixels.remove(&(*col, row));
        }
        for col in &lit_cols {
            self.lit_pixels
                .insert(((col + rotation) % self.num_cols, row));
        }
    }

    fn rotate_col(&mut self, col: usize, rotation: usize) {
        let lit_rows = self
            .lit_pixels
            .iter()
            .filter(|(c, _)| *c == col)
            .map(|(_, r)| *r)
            .collect::<Vec<_>>();
        for row in &lit_rows {
            self.lit_pixels.remove(&(col, *row));
        }
        for row in &lit_rows {
            self.lit_pixels
                .insert((col, (row + rotation) % self.num_rows));
        }
    }
}

impl Display for Screen {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        let display = (0..=self.num_rows)
            .map(|row| {
                (0..=self.num_cols)
                    .map(|col| {
                        if self.lit_pixels.contains(&(col, row)) {
                            '#'
                        } else {
                            ' '
                        }
                    })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n");

        write!(fmt, "{}", display)
    }
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = Regex::new(concat!(
            r"^((rect (?P<rect_cols>\d+)x(?P<rect_rows>\d+))",
            r"|(rotate row y=(?P<row>\d+) by (?P<row_rotation>\d+))",
            r"|(rotate column x=(?P<col>\d+) by (?P<col_rotation>\d+)))$",
        ))
        .unwrap();
        let captures = pattern.captures(s).ok_or_else(|| {
            Error::new(ErrorKind::InvalidData, "Invalid instruction")
        })?;

        if let Some((cols, rows)) = captures
            .name("rect_cols")
            .and_then(|s| s.as_str().parse().ok())
            .and_then(|cols| {
                captures
                    .name("rect_rows")
                    .and_then(|s| s.as_str().parse().ok())
                    .map(|rows| (cols, rows))
            })
        {
            Ok(Rectangle(cols, rows))
        } else if let Some((row, rotation)) = captures
            .name("row")
            .and_then(|s| s.as_str().parse().ok())
            .and_then(|row| {
                captures
                    .name("row_rotation")
                    .and_then(|s| s.as_str().parse().ok())
                    .map(|rotation| (row, rotation))
            })
        {
            Ok(RotateRow(row, rotation))
        } else if let Some((col, rotation)) = captures
            .name("col")
            .and_then(|s| s.as_str().parse().ok())
            .and_then(|col| {
                captures
                    .name("col_rotation")
                    .and_then(|s| s.as_str().parse().ok())
                    .map(|rotation| (col, rotation))
            })
        {
            Ok(RotateColumn(col, rotation))
        } else {
            Err(Error::new(ErrorKind::InvalidData, "Invalid instruction"))
        }
    }
}
