#! /bin/bash

day=$(date +%d | sed 's/^0*//')
curl -b $(cat ~/aoc_cookie) "https://adventofcode.com/2022/day/$day/input" > "day"$day".txt"