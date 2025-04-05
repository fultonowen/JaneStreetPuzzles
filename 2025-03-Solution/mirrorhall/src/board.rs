use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use tabled::{settings::Style};
use crate::cells::{CellState, MirrorState};
use crate::laser::Laser;
use crate::location::{Direction, Location};

#[derive(Debug, Clone)]
pub struct Board {
    pub mirrors : HashMap<Location, MirrorState>
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Board {
    fn new() -> Self {
        Self { mirrors : HashMap::<Location, MirrorState>::new() }
    }
    
    pub fn contains_mirror(&self, location: &Location) -> Option<&MirrorState> {
        self.mirrors.get(location)
    }
    
    pub fn add_mirror(&mut self, location: &Location, state: MirrorState) {
        self.mirrors.insert(location.clone(), state);
    }
    
    pub fn is_mirror_orthogonal(&self, location: &Location) -> bool {
        let directions = [ Direction::UP, Direction::LEFT, Direction::RIGHT, Direction::DOWN];
        let any_orthogonal = 
            directions.iter()
                .map(|dir| location.get_updated_location(dir))
                .map(|loc| {
                    match loc {
                        Some(x) => self.contains_mirror(&x),
                        None => None,
                    }
                })
                .any(|x| x.is_some());         
        any_orthogonal
    }
    
    pub fn clone_with_update(&self, location: &Location, state: MirrorState) -> Board {
        let mut cloned_board = self.clone();
        cloned_board.add_mirror(location, state);
        cloned_board
    }

    pub fn get_score(&self) -> BoardScore {
        let (top, bottom, left, right) =
            (1..=10usize)
                .fold((0u32, 0u32, 0u32, 0u32),
                      |acc, x| {
                          
                          let laser_top = Laser::new(Location(0, x), Direction::DOWN);
                          let laser_bottom = Laser::new(Location(11, x), Direction::UP);
                          let laser_left = Laser::new(Location(x, 0), Direction::RIGHT);
                          let laser_right = Laser::new(Location(x, 11), Direction::LEFT);
                          (
                              acc.0 + laser_top.compute_path(self),
                              acc.1 + laser_bottom.compute_path(self),
                              acc.2 + laser_left.compute_path(self),
                              acc.3 + laser_right.compute_path(self),
                          )
                      });
        BoardScore { top, bottom, left, right }
    }
}

#[derive(Debug)]
pub struct BoardScore {
    top    : u32,
    bottom : u32,
    left   : u32,
    right  : u32,
}

impl BoardScore {
    pub fn new(top: u32, bottom: u32, left: u32, right: u32) -> Self {
        Self { top, bottom, left, right }
    }
    
    pub fn score(&self) -> u64 {
        let top = self.top - (112u32 + 48u32 + 3087u32 + 9u32 + 1u32);
        let bottom = self.bottom - (2025u32 + 12u32 + 64u32 + 5u32 + 405u32);
        let left = self.left - (27u32 + 12u32 + 225u32);
        let right = self.right - (4u32 + 27u32 + 16u32);
        println!("TOP: {}, BOTTOM: {}, LEFT: {}, RIGHT: {}", top, bottom, left, right);
        top as u64 * bottom as u64 * left as u64 * right as u64
    }
}

#[derive(Debug, Clone)]
pub struct FinalBoard {
    pub values : [[CellState; 12]; 12]
}

impl From<Board> for FinalBoard {
    fn from(value: Board) -> Self {
        let mut res = FinalBoard::default();
        for (location, state) in value.mirrors.iter() {
            res.values[location.0][location.1] = CellState::Mirror(state.clone());
        }
        res
    }
}

impl Default for FinalBoard {
    fn default() -> Self {
        Self::new()
    }
}

impl FinalBoard {
    pub fn new() -> Self {
        Self {
            values : original_board(),
        }
    }
    
    pub fn output_solved(&mut self, board: &Board) {
        for x in 1..11 {
            let laser_top = Laser::new(Location(0, x), Direction::DOWN).compute_path(board);
            self.values[0][x] = CellState::Found(laser_top);
            let laser_bottom = Laser::new(Location(11, x), Direction::UP).compute_path(board);
            self.values[11][x] = CellState::Found(laser_bottom);
            let laser_left = Laser::new(Location(x, 0), Direction::RIGHT).compute_path(board);
            self.values[x][0] = CellState::Found(laser_left);
            let laser_right = Laser::new(Location(x, 11), Direction::LEFT).compute_path(board);
            self.values[x][11] = CellState::Found(laser_right);
        }
        println!("Solved board");
        println!("{}", self);
        
    }

}

impl Display for FinalBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut builder = tabled::builder::Builder::new();

        for row in &self.values {
            let row_data = row.iter().map(|x| x.to_string());
            builder.push_record(row_data);
        }
        let mut table = builder.build();
        table.with(Style::rounded());
        write!(f, "{}", table)
    }
}

pub fn original_board() -> [[CellState;12]; 12] {
    use crate::cells::CellState::{Unavailable, NotFound, Found, Empty};
    [
        [Unavailable, NotFound, NotFound, Found(112u32), NotFound, Found(48u32), Found(3087u32), Found(9u32), NotFound, NotFound, Found(1u32), Unavailable],
        [NotFound, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, NotFound],
        [NotFound, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Found(4u32)],
        [NotFound, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Found(27u32)],
        [Found(27u32), Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, NotFound],
        [NotFound, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, NotFound],
        [NotFound, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, NotFound],
        [NotFound, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Found(16u32)],
        [Found(12u32), Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, NotFound],
        [Found(225u32), Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, NotFound],
        [NotFound, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, Empty, NotFound],
        [Unavailable, Found(2025u32), NotFound, NotFound, Found(12u32), Found(64u32), Found(5u32), NotFound, Found(405u32), NotFound, NotFound, Unavailable],
    ]
}