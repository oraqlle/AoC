/// \brief AoC 2020 Day 01 Part 1 Solution
///
/// Author: Tyler Swann (oraqlle.net@gmail.com)
///
/// Date: 09/07/2023
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

    auto inv = std::unordered_set<std::int64_t> {};
    auto goal = std::int64_t { 2020 };
    auto result { 0uL };
    auto offset { 1uL };
    auto nums = std::vector<std::int64_t> {};

    std::ranges::copy(
        std::istream_iterator<std::int64_t>(file),
        std::istream_iterator<std::int64_t>(),
        std::back_inserter(nums));

    while (offset > 0uL) {
        auto start = std::next(nums.begin(), offset);
        auto first = *std::next(nums.begin(), offset - 1uL);
        auto target { goal - first };

        result = std::accumulate(start, nums.end(), std::int64_t { 0 }, [&inv, &target](auto acc, auto n) {
            inv.insert(target - n);
            return inv.contains(n) ? n * (target - n) : acc;
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
