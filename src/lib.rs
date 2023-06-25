use std::fmt;

pub struct Cells(Vec<CellState>);

impl Cells {
    
}

/// State of a Cell
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