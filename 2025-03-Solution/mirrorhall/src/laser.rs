use crate::board::Board;
use crate::cells::MirrorState;
use crate::location::{Direction, Location};
use crate::solver::SubProblem;

#[derive(Debug)]
pub struct Laser {
    location     : Location,
    direction    : Direction,
}

impl Laser {
    pub fn new(location: Location, direction: Direction) -> Self {
        Self {
            location,
            direction,
        }
    }

    pub fn compute_path(&self, board: &Board) -> u32 {
        let (mut total_length, mut curr_length) = (1u32, 0u32);
        let mut location_start = self.location.clone();
        let mut curr_direction = self.direction.clone();
        while let Some(loc_change) = location_start.update_location(&curr_direction) { // TODO: doesn't actually update location_start
            curr_length += 1;
            if let Some(state) = board.contains_mirror(&loc_change) {
                curr_direction = curr_direction.mirror_reflection(state);
                total_length *= curr_length;
                curr_length = 0u32;
            }
        }
        if curr_length > 1u32 { total_length *= curr_length; }
        total_length
    }
}

impl From<SubProblem> for Laser {
    fn from(value: SubProblem) -> Self {
        Laser::new(value.location.clone(), value.direction.clone())
    }
}

impl From<&SubProblem> for Laser {
    fn from(value: &SubProblem) -> Self {
        Laser::new(value.location.clone(), value.direction.clone())
    }
}
// input:
//      - Board
//      - Initial Location
//      - direction
//      - current_dist
//      - expected

#[derive(Debug, Clone, PartialEq)]
pub enum LaserStatus {
    Ready,
    Done,
}

#[derive(Debug, Clone)]
pub struct InflightLaser {
    pub curr_board   : Board,
    pub location     : Location,
    pub direction    : Direction,
    pub current_dist : u32,
    expected_dist    : u32,
    pub status       : LaserStatus
}

impl InflightLaser {
    pub fn new(curr_board: Board, location: Location, direction: Direction, current: u32, expected: u32, status: LaserStatus) -> Self{
        Self { curr_board, location, direction, current_dist: current, expected_dist: expected, status }
    }

    pub fn create_sub_paths(&self) -> Vec<Self> {
        let mut lasers = vec![];
        if self.status == LaserStatus::Done { return lasers; }
        let mut curr_location = self.location.clone();
        
        let distance_to_edge = curr_location.distance_to_edge(&self.direction);
        if self.current_dist * distance_to_edge as u32 == self.expected_dist {
            lasers.push(InflightLaser::new(
                self.curr_board.clone(), self.location.clone(), self.direction.clone(), 
                self.current_dist * distance_to_edge as u32, self.expected_dist, LaserStatus::Done)
            );
            return lasers
        }
        for step in 1u32..(distance_to_edge as u32) {
            curr_location = curr_location.get_updated_location(&self.direction).unwrap();
            if self.expected_dist % (self.current_dist * step) != 0u32 { continue; }
            if let Some(state) = self.curr_board.contains_mirror(&curr_location) {
                let new_direction = self.direction.mirror_reflection(state);
                lasers.push(
                    InflightLaser::new(
                        self.curr_board.clone(), curr_location.clone(), new_direction, 
                        self.current_dist * step, self.expected_dist, LaserStatus::Ready)
                );
            }
            else {
                if self.curr_board.is_mirror_orthogonal(&curr_location) { continue; }
                let new_board1 = self.curr_board.clone_with_update(&curr_location, MirrorState::PlacedLeftTop);
                lasers.push(
                    InflightLaser::new(
                        new_board1, curr_location.clone(), 
                        self.direction.mirror_reflection(&MirrorState::PlacedLeftTop),
                        self.current_dist * step, self.expected_dist, LaserStatus::Ready)
                );
                let new_board2 = self.curr_board.clone_with_update(&curr_location, MirrorState::PlacedLeftBottom);
                lasers.push(
                    InflightLaser::new(
                        new_board2, curr_location.clone(),
                        self.direction.mirror_reflection(&MirrorState::PlacedLeftBottom),
                        self.current_dist * step, self.expected_dist, LaserStatus::Ready)
                );
            }
        }
        lasers
    }

}
