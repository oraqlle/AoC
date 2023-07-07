/// \brief AoC 2020 Day 01 Part 1 Solution
///
/// Author: Tyler Swann (oraqlle.net@gmail.com)
///
/// Date: 08/07/2023
///
/// License: Apache-2.0 license
///
/// Copyright (c) 2023 - present
/// \file day-01-part-1.c

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main()
{
    char* path = "../day-01-input.txt";

    FILE* file = fopen(path, "r");

    if (file == NULL) {
        puts("Error opening file!");
        exit(1);
    }

    size_t goal = 2020uL;
    size_t current = 0uL;
    size_t seek = 0uL;
    size_t seen = 0uL;
    size_t result = 0uL;

    char buf[20];

    while (fgets(buf, sizeof buf, file)) {
        size_t sz = strcspn(buf, "\n");
        seen += sz + 1;
        buf[sz] = '\0';
        current = (size_t)strtoull(buf, NULL, 10);
        memset(&buf[0], 0, sizeof buf);

        rewind(file);
        while (fgets(buf, sizeof buf, file)) {
            buf[strcspn(buf, "\n")] = '\0';
            seek = (size_t)strtoull(buf, NULL, 10);
            memset(&buf[0], 0, sizeof buf);

            if (current + seek == goal && current != seek) {
                result = current * seek;
                break;
            }

            seek = 0uL;
        }

        if (result > 0uL)
            break;

        current = 0uL;
        fseek(file, (long)seen, SEEK_SET);
    }

    printf("Result: %zu\n", result);

    return 0;
}
