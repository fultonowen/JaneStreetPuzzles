#include <iostream>
#include <vector>
#include <unordered_set>
#include <string>
#include <numeric>

// 6 x 6 grid
// knights can have up to 8 moves if given space
// dx | dy
// 1  | 2
// 1  | -2
// -1 | 2
// -1 | -2
// 2  | -1
// 2  | 1
// -2 | -1
// -2 | 1
const std::vector<std::pair<int, int>> knight_moves{
        {1, 2}, {1, -2}, {-1, 2}, {-1, -2},
        {2, -1}, {2, 1}, {-2, -1}, {-2, 1}
};

char board[6][6] = {
        {'A','B','B','C','C','C'},
        {'A','B','B','C','C','C'},
        {'A','A','B','B','C','C'},
        {'A','A','B','B','C','C'},
        {'A','A','A','B','B','C'},
        {'A','A','A','B','B','C'}
};

int row_col_to_cell_number(int r, int c) {
    return r * 6 + c;
}

std::pair<int, int> cell_number_to_row_col(int val) {
    return std::make_pair(val / 6, val % 6);
}

// valid state transition still on board
bool is_valid_transition(int x, int y) {
    return x >= 0 && x < 6 && y >= 0 && y < 6;
}

std::string path_to_str(std::vector<int>& p) {
    std::string res;
    for (int cell : p) {
        res+= std::to_string(cell);
        res.push_back(',');
    }
    return res;
}

void backtrack(
        std::string& min_path,
        std::vector<int>& curr,
        std::vector<bool>& seen,
        int curr_cell,
        int dest_cell,
        int curr_score,
        const int arr[3],
        int& min_length_path
) {
    if (curr_score != 2024 && curr_cell == dest_cell) return;
    if (curr_score > 2024 || curr.size() > min_length_path) return;

    if (curr_cell == dest_cell) {
        if (int(curr.size()) < min_length_path) {
            min_path = path_to_str(curr);
            min_length_path = int(curr.size());
        }
        return;
    }

    auto [curr_x, curr_y] = cell_number_to_row_col(curr_cell);
    for (auto [dx, dy] : knight_moves) {
        int new_x{curr_x + dx}, new_y{curr_y + dy};

        if (!is_valid_transition(new_x, new_y) || seen[row_col_to_cell_number(new_x, new_y)]) continue;

        int new_cell = row_col_to_cell_number(new_x, new_y);
        seen[new_cell] = true;
        curr.push_back(new_cell);

        //update score based on whether state transition is occurring (ex. yes: A->B, no: A->A)
        if (board[curr_x][curr_y] == board[new_x][new_y]) {
            backtrack(min_path, curr, seen, new_cell, dest_cell, curr_score + arr[board[new_x][new_y]-'A'], arr, min_length_path);
        }
        else {
            backtrack(min_path, curr, seen, new_cell, dest_cell, curr_score * arr[board[new_x][new_y]-'A'], arr, min_length_path);
        }

        seen[new_cell] = false;
        curr.pop_back();
    }

}

std::string path_to_ans(const std::string& s) {
    //TODO: use stringstream to clean up
    std::string print_str;
    std::string small_str;
    for (char c : s) {
        if (c != ',') {
            small_str.push_back(c);
        } else {
            int val = std::stoi(small_str);
            auto [row, col] = cell_number_to_row_col(val);
            char left = '6'-row, right = 'a' + col;
            print_str.push_back(right);
            print_str.push_back(left);
            print_str.push_back(',');
            small_str = "";
        }
    }
    return print_str;
}

bool find_valid_paths(int start, int end, int A, int B, int C) {
    std::string ans;
    std::vector<bool> seen(36, false);
    std::vector<int> curr_path;
    curr_path.push_back(start);
    int arr[3] = {A,B,C};
    int min_path_length = INT32_MAX;
    backtrack(ans, curr_path, seen, start, end, arr[0], arr, min_path_length);
    std::cout << "Given A: " << A << " B: " << B << " C: " << C << " Shortest Path: " << (ans.empty() ? "None" : ans) << std::endl;
    std::cout << "Formatted path: " <<  path_to_ans(ans) << std::endl;
    return !ans.empty();
}

struct Comparator {
    constexpr bool operator() (std::vector<int>& lhs, std::vector<int>& rhs) {
        return std::accumulate(lhs.begin(), lhs.end(), 0) < std::accumulate(rhs.begin(), rhs.end(), 0);
    }
};

int main() {
//    std::priority_queue<std::vector<int>, std::vector<std::vector<int>>, Comparator> pq;
//    int min_sum_val = 6; // set for submission requirements
//    minimum possible could be any combination of values [1,2,3] for A+B+C total of 6 due to distinct requirement
//    std::cout << "a1 to f6" << std::endl;
//    for (int C = 1; C <=47; C++) {
//        for (int B = 1; B <=47; B++) {
//            if (C == B) continue;
//            for (int A = 1; A <= 47; A++) {
//                if ((A== B || A== C) || (A+B+C > min_sum_val)) continue;
//                if (find_valid_paths(30, 5, A, B, C)) {
//                    min_sum_val = std::min(A+B+C, min_sum_val);
//                    pq.push({A,B,C});
//                }
//            }
//        }
//    }
//
//    std::cout << "a6 to f1" << std::endl;
//    while (!pq.empty()) {
//        if (find_valid_paths(0, 35, pq.top()[0], pq.top()[1], pq.top()[2])) {
//            std::cout << "Match for A: " << pq.top()[0] << " B: " << pq.top()[1] << " C: " << pq.top()[2] << std::endl;
//        }
//        pq.pop();
//    }

    // "a6 to f1"
//    find_valid_paths(30, 5, 3,2,1);
//    find_valid_paths(30, 5, 3,1,2);
//    find_valid_paths(30, 5, 2,3,1);
//    find_valid_paths(30, 5, 2,1,3); // no
//    find_valid_paths(30, 5, 1,2,3); // no
    find_valid_paths(30, 5, 1,3,2);

    // "a1 to f6"
    find_valid_paths(0, 35, 3,2,1);
    find_valid_paths(0, 35, 3,1,2);
    find_valid_paths(0, 35, 2,3,1);
//    find_valid_paths(0, 35, 2,1,3); // no
//    find_valid_paths(0, 35, 1,2,3); // no
    find_valid_paths(0, 35, 1,3,2);

    return 0;
}

// valid tour // test example
// A = 1, B = 2, C = 253
// a1,b3,c5,d3,f4,d5,f6 -> ((1 + 1) * 2 + 2) * 253 + 253 + 253
//                      -> ((A+A) * B + B) *C + 2C = (2ABC + BC) +2C
// a6,c5,a4,b2,c4,d2,f1