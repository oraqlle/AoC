# \brief AoC 2020 Day 01 Part 1 Solution
#
# Author: Tyler Swann (oraqlle.net@gmail.com)
#
# Date: 09/07/2023
#
# License: Apache-2.0 license
#
# Copyright (c) 2023 - present
# \file day-01.exs

defmodule Day01 do
  @path Path.expand(Path.join("..", "day-01-input.txt"))

  def main do
    goal = 2020

    case File.read(@path) do
      {:error, reason} ->
        IO.puts(reason)
        exit(:error)

      {:ok, data} ->
        nums =
          data
          |> String.split("\n")
          |> Enum.map(&String.to_integer/1)

        {result, _} =
          nums
          |> Enum.reduce({0, 0}, fn _, {acc, offset} ->
            first = Enum.at(nums, offset)
            target = goal - first

            {inner_result, _} =
              nums
              |> Enum.drop(offset + 1)
              |> Enum.reduce({0, %MapSet{}}, fn n, {acc, inv} ->
                if MapSet.member?(inv, n),
                  do: {n * (target - n), MapSet.put(inv, target - n)},
                  else: {acc, MapSet.put(inv, target - n)}
              end)

            if inner_result != 0, do: {inner_result * first, offset}, else: {acc, offset + 1}
          end)

        IO.puts("Result: #{result}")
    end
  end
end

Day01.main()
