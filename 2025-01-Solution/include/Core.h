#ifndef INC_2025_01_SOLUTION_CORE_H
#define INC_2025_01_SOLUTION_CORE_H

#include <vector>
#include <unordered_set>

namespace GCD {
    uint32_t gcd(unsigned int a, unsigned int b);
    uint32_t gcd(const std::vector<unsigned int>& xs);
}

namespace Permutations {
    bool can_add(std::string& curr, const char& insert_val);
    void _find_valid_permutations(std::unordered_set<std::string>& res, std::string& curr, const std::vector<int>& numbers);
    void find_valid_permutations(std::unordered_set<std::string>& res, const std::vector<int>& numbers);
}

namespace Combinations {
    void build_combinations(std::vector<std::vector<int>>& ans, std::vector<int>& curr_path, std::vector<int>& nums, int i);
}

#endif //INC_2025_01_SOLUTION_CORE_H
