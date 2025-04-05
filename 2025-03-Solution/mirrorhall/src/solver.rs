use crate::board::Board;
use crate::laser::{InflightLaser, Laser, LaserStatus};
use crate::location::{Direction, Location};
use std::cmp::PartialEq;
use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
pub struct SubProblem {
    pub location       : Location,
    pub direction      : Direction,
    pub expected_value : u32,
}

impl SubProblem {
    pub fn new(r: usize, c: usize, expected_value: u32) -> Self {
        Self {
            location       : Location(r, c),
            direction      : Direction::from_board(r, c),
            expected_value,
        }
    }
}


#[derive(Debug)]
pub struct Solver;

impl Default for Solver {
    fn default() -> Self {
        Self::new()
    }
}

impl Solver {
    pub fn new() -> Self {
        Self
    }
    fn create_potential_boards(&self, board: &Board, problem: &SubProblem) -> Vec<Board> {
        let mut output : Vec<Board> = Vec::new();
        let initial_laser = 
            InflightLaser::new(board.clone(), problem.location.clone(), problem.direction.clone(),
                               1u32, problem.expected_value, LaserStatus::Ready);
        let mut stack : VecDeque<InflightLaser> = VecDeque::new();
        stack.push_back(initial_laser);
        while let Some(laser) = stack.pop_front() {
            
            if laser.current_dist == problem.expected_value && laser.status == LaserStatus::Done {
                output.push(laser.curr_board.clone());
            }
            if laser.current_dist > problem.expected_value { continue; }
            let is_greater_to_wall =  laser.location.distance_to_edge(&laser.direction) > 1;
            if laser.current_dist == problem.expected_value
                && is_greater_to_wall && laser.status == LaserStatus::Ready { continue; }
            let potential_lasers = laser.create_sub_paths();
            for x in potential_lasers {
                stack.push_back(x);
            }
        }
        output
    }
    
    pub fn solve_all(&self, initial_board: Board, problems: Vec<SubProblem>) -> Vec<Board> {
        let mut output : Vec<Board> = Vec::new();
        let mut stack : VecDeque<(usize, Board)> = VecDeque::new();
        stack.push_back((0, initial_board));
        // println!("HERE! {:?}", problems);
        while let Some((iter, curr_board)) = stack.pop_front() {
            if iter == problems.len() {
                output.push(curr_board.clone());
                continue;
            }
            let valid_boards_for_sub = self.create_potential_boards(&curr_board, &problems[iter]);
            // println!("{:?}", valid_boards_for_sub);
            let valid_boards_up_to : Vec<Board> = 
                valid_boards_for_sub.into_iter()
                    .filter(|x| check_subproblems_ok(x, &problems[..iter]))
                    .collect::<Vec<Board>>();
            // println!("{:?}", valid_boards_up_to);
            for x in valid_boards_up_to.iter() {
                stack.push_back((iter+1, x.to_owned()));
            }
        }
        
        output
    }

}

fn check_subproblem_ok(board: &Board, problem: &SubProblem) -> bool {
    let laser = Laser::from(problem);
    let path_length = laser.compute_path(board);
    if path_length != problem.expected_value { return false }
    true
}

pub fn check_subproblems_ok(board: &Board, problems: &[SubProblem]) -> bool {
    let is_ok = problems.iter()
        .all(|x| check_subproblem_ok(board, x));
    is_ok
}
