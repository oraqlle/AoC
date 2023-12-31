# \brief AoC 2022 Day 01 Part 1 Solution
#
# Author: Tyler Swann (oraqlle.net@gmail.com)
#
# Date: 28/06/2023
#
# License: Apache-2.0 license
#
# Copyright (c) 2023 - present
# \file day-01.exs

defmodule Day01 do
  @path Path.expand(Path.join("..", "day-01-input.txt"))

  def main do
    case File.read(@path) do
      {:error, reason} ->
        IO.puts(reason)
        exit(:error)

      {:ok, data} ->
        max =
          data
          |> String.split("\n\n")
          |> Enum.map(fn chunk ->
            chunk
            |> String.split("\n")
            |> Enum.map(fn str -> if str != "", do: String.to_integer(str), else: 0 end)
            |> Enum.sum()
          end)
          |> Enum.max()

        IO.puts("Result: #{max}")
    end
  end
end

Day01.main()
