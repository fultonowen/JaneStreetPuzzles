#include <vector>
#include <thread>

#include "Game.h"
#include "Solver.h"
#include "Core.h"


int main() {
    std::vector<int> main_set{0, 2, 5};
    std::vector<int> additional_set{1,3,4,6,7,8,9};

    std::vector<std::vector<int>> combinations;
    Combinations::build_combinations(combinations, main_set, additional_set, 0);

    std::vector<Solver> solvers;
    solvers.reserve(7);
    for (auto& combo : combinations) {
        auto game_config = GameConfiguration(combo);
        solvers.emplace_back(game_config);
    }

    std::vector<std::thread> tasks;
    for (auto& solver : solvers) {
        tasks.emplace_back(&Solver::run, solver);
    }

    for (auto& t : tasks) {
        t.join();
    }

    return 0;
}
