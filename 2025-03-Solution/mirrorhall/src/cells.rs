use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub enum MirrorState {
    PlacedLeftTop,
    PlacedLeftBottom,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CellState {
    Unavailable,
    Empty,
    Mirror(MirrorState),
    NotFound,
    Found(u32),
}

impl Display for CellState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            CellState::Unavailable => write!(f, "X"),
            CellState::Empty => write!(f, "O"),
            CellState::Mirror(MirrorState::PlacedLeftBottom) => write!(f, "/"),
            CellState::Mirror(MirrorState::PlacedLeftTop) => write!(f, "\\"),
            CellState::NotFound => write!(f, "."),
            CellState::Found(v) => write!(f, "{}", v),
        }
    }
}

impl std::ops::Add<CellState> for u32 {
    type Output = u32;

    fn add(self, rhs: CellState) -> Self::Output {
        match rhs {
            CellState::Found(x) => self + x,
            _ => panic!("Error: Could not add rhs ${rhs}"),
        }
    }
}

impl std::ops::Add<&CellState> for u32 {
    type Output = u32;

    fn add(self, rhs: &CellState) -> Self::Output {
        match rhs {
            CellState::Found(x) => self + x,
            _ => panic!("Error: Could not add rhs ${rhs}"),
        }
    }
}