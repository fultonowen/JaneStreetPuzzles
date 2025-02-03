#ifndef INC_2025_01_SOLUTION_SOLVER_H
#define INC_2025_01_SOLUTION_SOLVER_H

#include "Game.h"
#include <functional>
#include <iostream>

class Solver {
private:
    uint32_t max_gcd = 0;
    std::string best_solution;
    GameConfiguration g_conf;
    void solve(SingleSolverConfiguration& _conf);
public:
    explicit Solver(GameConfiguration& _g_conf) : g_conf(_g_conf) {};
    void run();
};

class SingleBoardSolver {
private:
    SingleSolverConfiguration conf;
    uint32_t max_gcd = 332; // pre-configuring to 332 to speed up runtime
    std::string max_gcd_solution;
    std::array<std::array<int, 10>, 9> rows{}, columns{}, boxes{};
    static int get_box_idx(int row, int col);
    void backtrack(int row, int col);
    bool place_only_one(int row, int col);
    bool could_place(int val, int row, int col);
    void place(int val, int row, int col);
    void remove(int row, int col);
    void place_next_numbers(int row, int col);
public:
    explicit SingleBoardSolver(SingleSolverConfiguration& config);
    void find_all_solutions();
    uint32_t get_max_gcd() const;
    std::string get_best_solution();
};


#endif //INC_2025_01_SOLUTION_SOLVER_H
