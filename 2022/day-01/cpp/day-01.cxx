/// \brief AoC 2022 Day 01 Solution
///
/// Author: Tyler Swann (oraqlle.net@gmail.com)
///
/// Date: 20/06/2023
///
/// License: Apache-2.0 license
///
/// Copyright (c) 2023 - present
/// \file day-01.cxx

#include <algorithm>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <numeric>
#include <ranges>
#include <string>
#include <string_view>
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

    auto data = std::vector<std::string> {};
    auto line = std::string {};

    while (std::getline(file, line)) {
        data.push_back(std::move(line));
    }

    auto sums = data
        | std::views::split(""s)
        | std::views::transform([](auto const& rng) {
              auto nums = rng
                  | std::views::transform([](auto const x) { return std::stoull(x); })
                  | std::views::common;
              return std::accumulate(nums.begin(), nums.end(), 0uL);
          })
        | std::views::common;

    auto result = *std::max_element(sums.begin(), sums.end());

    std::cout << "Result: " << result << std::endl;

    return 0;
}
