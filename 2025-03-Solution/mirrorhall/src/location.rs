use crate::cells::MirrorState;

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
pub struct Location(pub usize, pub usize);

#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    UP, 
    DOWN,
    LEFT,
    RIGHT
}

impl Location {
    pub fn new(x: usize, y: usize) -> Self {
        Self(x, y)
    }

    pub fn get_updated_location(&self, direction: &Direction) -> Option<Location> {
        match direction {
            Direction::UP => {
                let x = self.0.checked_sub(1)?;
                Some(Location(x, self.1))
            },
            Direction::DOWN => {
                if self.0 + 1 > 11 { None }
                else { Some(Location(self.0 + 1, self.1)) }
            },
            Direction::LEFT => {
                let y = self.1.checked_sub(1)?;
                Some(Location(self.0, y))
            },
            Direction::RIGHT => {
                if self.1 + 1 > 11 { None }
                else { Some(Location(self.0, self.1 + 1)) }
            },
        }
    }
    
    pub fn update_location(&mut self, direction:& Direction) -> Option<Location> {
        let new_location = self.get_updated_location(direction);
        if let Some(x) = &new_location {
            self.0 = x.0;
            self.1 = x.1;
        }
        new_location
    }
    
    pub fn distance_to_edge(&self, direction: &Direction) -> usize {
        match direction {
            Direction::UP => self.0,
            Direction::DOWN => 11 - self.0,
            Direction::RIGHT => 11 - self.1,
            Direction::LEFT => self.1,
        }
    }
    
    
}

impl Direction {
    pub fn from_board(start_r: usize, start_c: usize) -> Self {
        match (start_r, start_c) {
            (0, _) => Direction::DOWN,
            (_, 0) => Direction::RIGHT,
            (11, _) => Direction::UP,
            (_, 11) => Direction::LEFT,
            (_, _) => panic!("Error: Invalid start location for direction."),
        }
    }

    fn mirror_placed_left_top(&self) -> Direction {
        match self {
            Direction::UP => Direction::LEFT,
            Direction::DOWN => Direction::RIGHT,
            Direction::LEFT => Direction::UP,
            Direction::RIGHT => Direction::DOWN,
        }
    }

    fn mirror_placed_left_bottom(&self) -> Direction {
        match self {
            Direction::UP => Direction::RIGHT,
            Direction::DOWN => Direction::LEFT,
            Direction::LEFT => Direction::DOWN,
            Direction::RIGHT => Direction::UP,
        }
    }
    
    pub fn mirror_reflection(&self, mirror_state: &MirrorState) -> Direction {
        match mirror_state {
            MirrorState::PlacedLeftTop => self.mirror_placed_left_top(),
            MirrorState::PlacedLeftBottom => self.mirror_placed_left_bottom(),
        }
    }
}
