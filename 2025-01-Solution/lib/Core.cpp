#include "Core.h"
#include <vector>
#include <unordered_set>

uint32_t GCD::gcd(uint32_t a, uint32_t b) {
    return a == 0 ? b : gcd(b % a, a);
}

uint32_t GCD::gcd(const std::vector<uint32_t>& xs) {
    uint32_t result = xs[0];
    for (size_t i = 1; i < xs.size(); i++) {
        result = gcd(xs[i], result);
        if (result == 1) return 1;
    }
    return result;
}

bool Permutations::can_add(std::string& curr, const char& insert_val) {
    if (curr.find(insert_val) != std::string::npos) return false;

    if (insert_val != '0' && insert_val != '2' && insert_val != '5') {
        return curr.find(insert_val) == std::string::npos;
    }
    int size = curr.size();

    if (insert_val == '2') return size == 7;
    else if (insert_val == '0') return size == 0 || size == 1 || size == 3 || size == 5 || size == 7 || size == 8;
    else return size == 0 || size == 1 || size == 2 || size == 3 || size == 4 || size ==5;
}

void Permutations::_find_valid_permutations(std::unordered_set<std::string>& ans, std::string& curr, const std::vector<int>& numbers) {
    if (curr.size() > 9) return;
    if (curr.size() == 9 && !ans.contains(curr)) {
        ans.insert(curr);
        return;
    }
    for (auto& item : numbers) {
        char v = item + '0';
        if (can_add(curr, v)) {
            curr.push_back(v);
            _find_valid_permutations(ans, curr, numbers);
            curr.pop_back();
        }
    }
}

void Permutations::find_valid_permutations(std::unordered_set<std::string> &res, const std::vector<int> &numbers) {
    std::string curr;
    _find_valid_permutations(res, curr, numbers);
}

void Combinations::build_combinations(std::vector<std::vector<int>>& ans, std::vector<int>& curr_path, std::vector<int>& nums, int i) {
    if (curr_path.size() == 9) {
        ans.push_back(curr_path);
        return;
    }
    for (size_t j = i; j < nums.size(); j++) {
        curr_path.push_back(nums[j]);
        build_combinations(ans, curr_path, nums, j + 1);
        curr_path.pop_back();
    }
}
