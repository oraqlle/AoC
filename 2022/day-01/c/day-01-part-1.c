/// \brief AoC 2022 Day 01 Part 1 Solution
///
/// Author: Tyler Swann (oraqlle.net@gmail.com)
///
/// Date: 09/07/2023
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

    size_t sum = 0;
    size_t max = 0;

    char current[20];

    while (fgets(current, sizeof current, file)) {

        if (strcmp(current, "\n") == 0) {
            if (max < sum)
                max = sum;

            sum = 0;
        } else {
            current[strcspn(&current[0], "\n")] = '\0';
            sum += (size_t)strtoull(&current[0], NULL, 10);
        }
    }

    printf("Result: %zu\n", max);
    fclose(file);

    return 0;
}
