use lucky_minesweeper::*;

fn main() {
    const ROWS: usize = 9;
    const COLS: usize = 9;
    const MINES: usize = 10;
    const ITERS: usize = 100_000_000;
    // set to x to output a progress report every x iterations. 1 << 23 is a good value
    const PROGRESS: u64 = 1 << 23;

    println!(
        "{}",
        pretty_output(test::<ROWS, COLS, MINES, ITERS, PROGRESS>())
    );
}
