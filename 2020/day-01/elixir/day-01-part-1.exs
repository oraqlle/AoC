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
        {result, _} =
          data
          |> String.split("\n")
          |> Enum.map(&String.to_integer/1)
          |> Enum.reduce({0, %MapSet{}}, fn n, {acc, inv} ->
            if MapSet.member?(inv, n),
              do: {n * (goal - n), MapSet.put(inv, goal - n)},
              else: {acc, MapSet.put(inv, goal - n)}
          end)

        IO.puts("Result: #{result}")
    end
  end
end

Day01.main()
