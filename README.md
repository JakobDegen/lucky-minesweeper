## Lucky Minesweeper

Simulation tool that answers too questions:
 1. How lucky do you need to be to solve a minesweeper board in one guess?
 2. Where should you guess if that's your goal?

This was a one (and a half) day project, so don't expect much. All board sizes are supported, but
those and related parameters are const-generic (yay perf!) so if you want a different size, change
the values in `main.rs` and re-compile. I use a couple more advanced const-generic features, so
compiling requires nightly.

Should be decently (but not insanely) optimized. I'm not looking to optimize this much more right
now unless someone has any suggestions for getting an order of at least an order of magnitude of
performance out. Probably this will require some changed strategy.

## Results

About one out of 70,000 beginner sized minesweeper boards (9x9 with 10 mines) can be solved in a
single guess. Surprisingly, the best place to guess is to do this is in a corner of the board.
A guess in the corner will give you about a one in 180,000 chance of instantly solving the puzzle.
Guessing diagonal to a corner is the worst option, where you're reduced to about a one in 4,000,000
chance of solving the board.

I don't have enough compute to have found even a single instantly solvable board in Intermediate or
Advanced sizes. Would love to hear from anyone who finds one though!
