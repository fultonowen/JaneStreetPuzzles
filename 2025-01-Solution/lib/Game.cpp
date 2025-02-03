
#include "Core.h"
#include "Game.h"

PlayingBoard::PlayingBoard(const StartingBoard& other, const std::string& first_row) {
    for (int i = 0; i < 9; i++) {
        for (int j = 0; j < 9; j++) {
            values[i][j] = other.board[i][j];
        }
        values[0][i] = static_cast<int>(first_row[i] - '0');
    }
}

std::string PlayingBoard::to_string() {
    std::string view;
    for (auto& value : values) {
        for (int& item : value) {
            if (item == -1) view.push_back('.');
            else view.push_back(item + '0');
        }
        view.push_back('\n');
    }
    return view;
}

uint32_t PlayingBoard::row_value(int row_idx) {
    uint32_t row_val = 0;
    int dec_place = 0;
    for (int col = 8; col >=0; col--) {
        row_val += values[row_idx][col] * (uint32_t) std::pow(10, dec_place++);
    }
    return row_val;
}

uint32_t PlayingBoard::gcd_up_until_row(int row_idx) {
    uint32_t res = row_value(0);
    for (int i = 1; i <= row_idx; i++) {
        res = GCD::gcd(res, row_value(i));
        if (res == 1) return 1;
    }
    return res;
}

uint32_t PlayingBoard::gcd() {
    std::vector<uint32_t> row_values;
    row_values.reserve(9);
    uint32_t row_val;
    for (int row = 0; row < 9; row++) {
        row_val = PlayingBoard::row_value(row);
        row_values.push_back(row_val);
    }

    return GCD::gcd(row_values);
}


SingleSolverConfiguration::SingleSolverConfiguration(GameConfiguration& _g_config, const std::string& col)
        : numbers(_g_config.numbers), board(_g_config.board, col) {}