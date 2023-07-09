/// \brief AoC 2020 Day 01 Part 1 Solution
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

    size_t seen = 0uL;
    long long goal = 2020LL;
    long long current = 0LL;
    long long seek = 0LL;
    long long result = 0LL;

    char buf[20];

    while (fgets(buf, sizeof buf, file)) {
        long long sz = (long long)strcspn(buf, "\n");
        seen += sz + 1LL;
        buf[sz] = '\0';
        current = (long long)strtoll(buf, NULL, 10);
        memset(&buf[0], 0, sizeof buf);

        rewind(file);
        while (fgets(buf, sizeof buf, file)) {
            buf[strcspn(buf, "\n")] = '\0';
            seek = (long long)strtoll(buf, NULL, 10);
            memset(&buf[0], 0, sizeof buf);

            if (current + seek == goal && current != seek) {
                result = current * seek;
                break;
            }

            seek = 0LL;
        }

        if (result > 0LL)
            break;

        current = 0LL;
        fseek(file, (long)seen, SEEK_SET);
    }

    printf("Result: %lld\n", result);
    fclose(file);

    return 0;
}
