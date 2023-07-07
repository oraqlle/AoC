/// \brief AoC 2020 Day 01 Part 1 Solution
///
/// Author: Tyler Swann (oraqlle.net@gmail.com)
///
/// Date: 08/07/2023
///
/// License: Apache-2.0 license
///
/// Copyright (c) 2023 - present
/// \file day-01-part-1.cxx

#include <algorithm>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <iterator>
#include <numeric>
#include <ranges>
#include <unordered_set>
#include <vector>

using namespace std::literals;
namespace fs = std::filesystem;

auto main(int argc, char* argv[]) -> int
{

    auto fpath = fs::weakly_canonical(argv[0]).parent_path() / ".."sv / "day-01-input.txt"sv;

    auto file = std::ifstream(fpath);

    if (!file.is_open()) {
        std::cerr << "Failed to open file! " << std::endl;
        std::exit(1);
    }

    auto nums = std::vector<std::size_t> {};

    std::ranges::copy(
        std::istream_iterator<std::size_t>(file),
        std::istream_iterator<std::size_t>(),
        std::back_inserter(nums));

    auto inv = std::unordered_set<std::size_t> {};
    auto goal { 2020uL };
    auto result { 0uL };
    auto offset { 1uL };

    while (offset > 0uL) {

        auto start = std::next(nums.begin(), offset);
        auto first { *start };
        auto target { goal - first };

        result = std::accumulate(start, nums.end(), 0uL, [&inv, &target](auto acc, auto n) {
            if (inv.contains(n))
                return acc = n * (target - n);

            inv.insert(target - n);
            return acc;
        });

        if (result != 0uL) {
            result *= first;
            break;
        }

        offset += 1uL;
    }

    std::cout << "Result: " << result << std::endl;

    return 0;
}
