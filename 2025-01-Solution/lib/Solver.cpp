
#include "Solver.h"
#include "Game.h"
#include "Core.h"

#include <iostream>
#include <unordered_set>
#include <string>
#include <thread>

void Solver::solve(SingleSolverConfiguration& _config) {
    SingleBoardSolver solver = SingleBoardSolver(_config);
    solver.find_all_solutions();
    uint32_t max_gcd_for_board = solver.get_max_gcd();
    if (max_gcd_for_board > max_gcd) {
        max_gcd = max_gcd_for_board;
        best_solution = solver.get_best_solution();
    }
}

void Solver::run() {
    std::unordered_set<std::string> perms;
    Permutations::find_valid_permutations(perms, g_conf.numbers);
    for (const std::string& perm : perms) {
        SingleSolverConfiguration s_conf = SingleSolverConfiguration(g_conf, perm);
        Solver::solve(s_conf);
    }

    std::cout << "FINAL FOR SOLVER from thread (" << std::this_thread::get_id() << ")\n";
    std::cout << "----------------------------------------------------\n";
    std::cout << "Max GCD: " << max_gcd << "\n";
    std::cout << "Max Solution:\n";
    std::cout << best_solution << std::endl;
}

SingleBoardSolver::SingleBoardSolver(SingleSolverConfiguration& _s_conf) : conf(std::move(_s_conf)) {}

int SingleBoardSolver::get_box_idx(int row, int col) {
    return (row / 3 ) * 3 + col / 3;
}

void SingleBoardSolver::find_all_solutions() {
    for (int i = 0; i < 9; i++) {
        for (int j = 0; j < 9; j++) {
            if (conf.board.values[i][j] != -1) {
                place(conf.board.values[i][j], i, j);
            }
        }
    }
    backtrack(0, 0);
}


bool SingleBoardSolver::place_only_one(int row, int col) { // place number if there's only one possible values
    int placed_num = -1, count = 0;
    for (int& num : conf.numbers) {
        if (could_place(num, row, col)) {
            placed_num = num;
            count++;
        }
    }
    if (count == 1) {
        place(placed_num, row, col);
        return true;
    }
    return false;
}

bool SingleBoardSolver::could_place(int val, int row, int col) {
    return rows[row][val] + columns[col][val] + boxes[get_box_idx(row, col)][val] == 0;
}

void SingleBoardSolver::place(int val, int row, int col) {
    rows[row][val]++;
    columns[col][val]++;
    boxes[get_box_idx(row, col)][val]++;
    conf.board.values[row][col] = val;
}

void SingleBoardSolver::remove(int row, int col) {
    int remove_val = conf.board.values[row][col];
    rows[row][remove_val]--;
    columns[col][remove_val]--;
    boxes[get_box_idx(row, col)][remove_val]--;
    conf.board.values[row][col] = -1;
}

void SingleBoardSolver::backtrack(int row, int col) {
    if (conf.board.values[row][col] == -1) {
        for (int& d : conf.numbers) {
            if (could_place(d, row, col)) {
                if (!place_only_one(row, col)) place(d, row, col);
                place_next_numbers(row, col);
                remove(row, col);
            }
        }
    } else place_next_numbers(row, col);
}

void SingleBoardSolver::place_next_numbers(int row, int col) {
    if (col == 8 && row == 8) {
        uint32_t curr_sol_gcd = conf.board.gcd();
        if (curr_sol_gcd > max_gcd) {
            max_gcd = curr_sol_gcd;
            max_gcd_solution = conf.board.to_string();
        }
    }
    else {
        if (col == 8) {
            if (conf.board.gcd_up_until_row(row) > max_gcd)
                backtrack(row + 1, 0);
        }
        else backtrack(row, col + 1);
    }
}

uint32_t SingleBoardSolver::get_max_gcd() const {
    return max_gcd;
}

std::string SingleBoardSolver::get_best_solution() {
    return max_gcd_solution;
}
