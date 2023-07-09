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
    auto nums = std::vector<std::int64_t> {};

    std::ranges::copy(
        std::istream_iterator<std::int64_t>(file),
        std::istream_iterator<std::int64_t>(),
        std::back_inserter(nums));

    auto result = std::accumulate(nums.begin(), nums.end(), std::int64_t { 2020 }, [&inv, &goal](auto acc, auto n) {
        inv.insert(goal - n);
        return inv.contains(n) ? n * (goal - n) : acc;
    });

    std::cout << "Result: " << result << std::endl;

    return 0;
}
