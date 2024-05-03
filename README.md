# leetcode

LeetCode problems solved as a part of the Calgary Rust meetup.

## About the repository

This repository is a [Cargo workplace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) that contains solutions to LeetCode problems, which have been solved by the members of Calgary Rust or are to be solved, if the directory for the problem is new.

## Problem directories

The directories matching the [regular expression](https://en.wikipedia.org/wiki/Regular_expression) `p\d+` (e.g. `p1450`) are [stipulatively](https://en.wikipedia.org/wiki/Stipulative_definition) called problem directories. Each problem directory contains

1. a `README.md` file that describes the problem
2. the solution(s) to the problem - a [Rust package](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html), which is a member of the Cargo workplace.
