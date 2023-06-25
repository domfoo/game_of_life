use std::fmt;

/// a Cell canvas
pub struct Cells(Vec<Vec<CellState>>);

impl Cells {
    /// create a new Cell canvas
    pub fn new(width: usize, height: usize) -> Self {
        Self { 0: vec![vec![CellState::Dead; width]; height] }
    }

    /// create the new generation on a new canvas
    pub fn evolve(&self) -> Self {
        Self::new(40, 20)
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
#[derive(Clone)]
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