# Sudoku solver

A simple sudoku solver that uses a recursive backtracking algorithm. I store sudoku boards as 9x9 arrays of 8-bit integers and sets of used numbers in each row, column, and square as arrays of 16-bit integers.

On my machine it solves [this](https://www.flickr.com/photos/npcomplete/2361922699) board in approximately 60ms.
The example [from Wikipedia](https://commons.wikimedia.org/wiki/File:Sudoku_Puzzle_by_L2G-20050714_standardized_layout.svg#/media/File:Sudoku_Puzzle_by_L2G-20050714_standardized_layout.svg) takes 240 nano seconds to complete.

I experimented with not copying the board on every iteration and with parallelizing checking each legal move, but they turned out to be slower.
