use mirrorhall::board::{original_board, Board, FinalBoard};
use mirrorhall::cells::{CellState, MirrorState};
use mirrorhall::location::Location;
use mirrorhall::solver::{Solver, SubProblem};

fn main() {
    
    let board_grid = original_board();
    
    let mut constrained_problems : Vec<SubProblem> = vec![];
    
    for i in 0..12 {
        for j in 0..12 {
            
            if let CellState::Found(x) = board_grid[i][j] {
                if x != 1u32 {
                    constrained_problems.push(SubProblem::new(i, j, x));
                }
            }
        }
    }
    
    constrained_problems.sort_by(|a, b| a.expected_value.cmp(&b.expected_value));
    let mut board2 = Board::default();
    board2.add_mirror(&Location(1, 10), MirrorState::PlacedLeftTop);
    let solver = Solver;
    let f = solver.solve_all(board2, constrained_problems);
    
    println!("Solved boards!");
    for b in f {
        let score = b.get_score();
        let mut g = FinalBoard::from(b.clone());
        g.output_solved(&b);
        let answer = score.score();
        println!("{:?}", score);
        println!("Final Answer: {}", answer);
    }

}
