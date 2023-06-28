/// \brief AoC 2022 Day 01 Part 1 Solution
///
/// Author: Tyler Swann (oraqlle.net@gmail.com)
///
/// Date: 28/06/2023
///
/// License: Apache-2.0 license
///
/// Copyright (c) 2023 - present
/// \file day-01-part-1.zig
const std = @import("std");

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();

    var gpa = std.heap.GeneralPurposeAllocator(.{ .safety = true }){};
    defer _ = gpa.deinit();
    var alloc = &gpa.allocator();

    var path_buffer: [std.fs.MAX_PATH_BYTES]u8 = undefined;
    const path = try std.fs.realpath("../day-01-input.txt", &path_buffer);

    const file = try std.fs.openFileAbsolute(path, .{ .mode = .read_only });
    defer file.close();

    const buffer_size = 20000;
    const file_buffer = try file.readToEndAlloc(alloc.*, buffer_size);
    defer alloc.free(file_buffer);

    var iter_chunks = std.mem.split(u8, file_buffer, "\n\n");

    var sum: usize = 0;
    var max: usize = 0;

    while (iter_chunks.next()) |chunk| {
        var strs = std.mem.split(u8, chunk, "\n");

        while (strs.next()) |numStr| {
            if (!std.mem.eql(u8, numStr, "")) {
                const num = try std.fmt.parseUnsigned(usize, numStr, 10);
                sum += num;
            }
        }

        max = @max(sum, max);
        sum = 0;
    }

    try stdout.print("Result: {d}\n", .{max});
}
