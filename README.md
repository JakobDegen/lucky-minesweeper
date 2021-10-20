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
