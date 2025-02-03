#ifndef INC_2025_01_SOLUTION_GAME_H
#define INC_2025_01_SOLUTION_GAME_H

#include <vector>
#include <string>

struct StartingBoard {
    int board[9][9] = {
            {-1, -1, -1, -1, -1, -1, -1, 2, -1},
            {-1, -1, -1, -1,  2, -1, -1, -1, 5},
            {-1,  2, -1, -1, -1, -1, -1, -1, -1},
            {-1, -1,  0, -1, -1, -1, -1, -1, -1},
            {-1, -1, -1, -1, -1, -1, -1, -1, -1},
            {-1, -1, -1,  2, -1, -1, -1, -1, -1},
            {-1, -1, -1, -1,  0, -1, -1, -1, -1},
            {-1, -1, -1, -1, -1,  2, -1, -1, -1},
            {-1, -1, -1, -1, -1, -1,  5, -1, -1},
    };
    StartingBoard() = default;
};

struct PlayingBoard {
    int values[9][9]{};
    PlayingBoard() = delete;
    PlayingBoard(const StartingBoard& other, const std::string& last_column);
    std::string to_string();
    uint32_t gcd();
    uint32_t row_value(int row_idx);
    uint32_t gcd_up_until_row(int row_idx);
};

struct GameConfiguration {
    std::vector<int> numbers;
    StartingBoard board;
    explicit GameConfiguration(std::vector<int>& _nums) : numbers(std::move(_nums)) {}
};

struct SingleSolverConfiguration {
    std::vector<int> numbers;
    PlayingBoard board;
    explicit SingleSolverConfiguration(GameConfiguration& _g_config, const std::string& col);
};

#endif //INC_2025_01_SOLUTION_GAME_H
