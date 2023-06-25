#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


use std::fmt;

/// a Cell canvas
#[derive(Debug)]
pub struct Cells(Vec<Vec<CellState>>);

impl Cells {
    /// create a new Cell canvas
    pub fn new(width: usize, height: usize) -> Self {
        Self { 0: vec![vec![CellState::Dead; width]; height] }
    }
}

impl fmt::Display for Cells {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.0 {
            for cell in row {
                write!(f, "{cell}")?;
            }
            write!(f,"\n")?;
        }
        write!(f, "")
    }
}

/// State of a Cell
#[derive(Clone, Debug)]
enum CellState {
    Alive,
    Dead
}

impl fmt::Display for CellState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match &self {
            CellState::Alive => "O",
            CellState::Dead => "-"
        })
    }
}