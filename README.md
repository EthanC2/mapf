# mapf
Maps mathematical expressions to input from STDIN.

# Planned Features

## General
- Support binary operators
- Support floating-point numbers

## Options
- Ignore non-numeric (garbage) input like NAN and INF
- Panic instead of ignoring garbage
- Change output base
- Accumulate based on math operators (+, -, *, /)
- Support floating-point numbers

## Filtering
- Filtering by boolean predicates
- Filtering by multiple predicates
- Filtering before AND/OR after mapping function

## Optimizations
- Do not parse the user input function each time. Instead, dynamically build a function by chaining simple functions

## Misc
- All calculations performed as strings, allowed to go beyond 2^64 - 1


# Notes to Self
x / 10 - 20 + 30
/ x 10
- 20
+ 30