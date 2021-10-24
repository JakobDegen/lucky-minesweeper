#![allow(incomplete_features)] // Stop me
#![feature(generic_const_exprs)]

use itertools::Itertools;
use rand::distributions::{Distribution, Uniform};
use rand::{rngs::SmallRng, SeedableRng};
use rayon::prelude::*;
use std::cell::RefCell;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Square {
    // This square is a mine
    Mine,
    // Has no adjacent mines
    Open,
    // Has adjacent mines
    Closed,
}
use Square::*;

/// Pretty prints the board
pub fn display<const N: usize, const M: usize>(a: [[Square; M]; N]) -> String {
    let mut s = String::new();
    for i in a {
        for c in i {
            s.push(match c {
                Mine => 'M',
                Open => '.',
                Closed => 'x',
            })
        }
        s.push('\n');
    }
    s
}

/// Pretty prints the output provided by the `test` function
pub fn pretty_output<const N: usize, const M: usize>(
    (a, total): ([[usize; M]; N], usize),
) -> String {
    format!(
        "Total: {}\n{}",
        total,
        a.iter().map(|i| i.iter().format("\t")).format("\n")
    )
}

/// Check whether the given NxM mine sweeper board can be solved in a single guess
pub fn check<const N: usize, const M: usize>(mut board: [[Square; M]; N]) -> bool {
    // let board_dup = board;
    std::thread_local!(
        static SCRATCH: RefCell<Vec<(usize, usize)>> = RefCell::new(Vec::new())
    );
    SCRATCH.with(|s| {
        // Does not panic as this function is non-reentrant
        let mut scratch = s.try_borrow_mut().unwrap();
        let start = (0..N)
            .cartesian_product(0..M)
            .find(|&(a, b)| board[a][b] == Open)
            .unwrap();
        scratch.push(start);
        // This is a DFS of the open squares. We set everything that's reachable to `Mine`
        while let Some((x, y)) = scratch.pop() {
            // We may have searched this square already
            if board[x][y] == Mine {
                continue;
            }
            board[x][y] = Mine;
            let istart = if x > 0 { x - 1 } else { x };
            let iend = if x < N - 1 { x + 1 } else { x };
            let jstart = if y > 0 { y - 1 } else { y };
            let jend = if y < M - 1 { y + 1 } else { y };
            for i in istart..=iend {
                for j in jstart..=jend {
                    if board[i][j] == Open {
                        // We do not set this to be a Mine here; we'll set it when we pop it out of
                        // the DFS stack
                        scratch.push((i, j));
                    } else {
                        // Shouldn't ever be able to find a `Mine`, so `board[i][j] == Closed`
                        board[i][j] = Mine;
                    }
                }
            }
        }
    });

    board.iter().all(|x| x.iter().all(|&i| i == Mine))
}

/// Generate a NxM minesweeper board with K mines
pub fn generate<const N: usize, const M: usize, const K: usize>() -> [[Square; M]; N]
where
    [(); N + 2]: ,
    [(); M + 2]: ,
{
    // Making the board a little bit bigger reduces our need to worry about bounds checks and gives
    // us back 10% perf
    let mut board = [[Open; M + 2]; N + 2];
    let mut count = 0;
    // Hitting the thread rng each board we generate turns out not to be *too* bad, the total time
    // we spend in the RNG is only about 10% of this function, and most of that is doing the
    // sampling.
    let mut rng = SmallRng::from_rng(&mut rand::thread_rng()).unwrap();
    let between_n = Uniform::from(1..N + 1);
    let between_m = Uniform::from(1..M + 1);
    while count < K {
        let (x, y) = (between_n.sample(&mut rng), between_m.sample(&mut rng));
        if board[x][y] == Mine {
            continue;
        }
        count += 1;
        board[x][y] = Mine;

        for i in x - 1..=x + 1 {
            let chunk = unsafe { board.get_unchecked_mut(i).get_unchecked_mut(y - 1..=y + 1) };
            for entry in chunk {
                if *entry != Mine {
                    *entry = Closed;
                }
            }
        }
    }
    let mut out = [[Open; M]; N];
    for i in 0..N {
        for j in 0..M {
            out[i][j] = board[i + 1][j + 1];
        }
    }

    out
}

/// Generates T boards of NxM minesweeper with K mines and outputs:
///  - an NxM array where entry i,j is the number of times that initially guessing i,j would
///    immediately solve the puzzle
///  - a usize containing the total number of puzzles that could be solved by a single guess
/// If PROG is non-zero, then the simulation will print a message to stdout every time it completes
/// PROG tests
pub fn test<const N: usize, const M: usize, const K: usize, const T: usize, const PROG: u64>(
) -> ([[usize; M]; N], usize)
where
    [(); N + 2]: ,
    [(); M + 2]: ,
{
    let progress = std::sync::atomic::AtomicU64::new(0);
    (0..T)
        .into_par_iter()
        .map(|_| {
            let b = generate::<N, M, K>();
            if check(b) {
                // We box things here because otherwise Rayon overflows our stack. This is not a
                // perf hit, as the true branch of `check(b)` is extremely cold for most reasonable
                // parameters.
                Some(Box::new((b.map(|i| i.map(|x| (x == Mine).into())), 1)))
            } else {
                None
            }
        })
        // Do progress reporting
        .inspect(|_| {
            if PROG != 0 {
                let v = progress.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if v % PROG == 0 {
                    println!("Progress: {}", v);
                }
            }
        })
        .reduce(
            || None,
            |a, b| match (a, b) {
                (None, None) => None,
                (Some(x), None) => Some(x),
                (None, Some(x)) => Some(x),
                (Some(mut a), Some(b)) => {
                    for i in 0..N {
                        for j in 0..M {
                            (a.0)[i][j] += (b.0)[i][j];
                        }
                    }
                    a.1 += b.1;
                    Some(a)
                }
            },
        )
        .map(|a| (a.0, a.1)) // unboxes
        .unwrap_or_else(|| ([[0; M]; N], 0))
}

/// Identical to test, except does not use Rayon and is single threaded. The only reason to prefer
/// this is because benchmarking while also running Rayon is hard (have you *seen* a
/// `cargo flamegraph` output?)
pub fn test_single_threaded<
    const N: usize,
    const M: usize,
    const K: usize,
    const T: usize,
    const PROG: u64,
>() -> ([[usize; M]; N], usize)
where
    [(); N + 2]: ,
    [(); M + 2]: ,
{
    let progress = std::sync::atomic::AtomicU64::new(0);
    (0..T)
        .into_iter()
        .map(|_| {
            let b = generate::<N, M, K>();
            if check(b) {
                // We box things here to keep from overflowing the stack. This is not a perf hit, as
                // the allocation is only done if we actually find a solution
                Some(Box::new((b.map(|i| i.map(|x| (x == Mine).into())), 1)))
            } else {
                None
            }
        })
        // Do progress reporting
        .inspect(|_| {
            if PROG != 0 {
                let v = progress.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                if v % PROG == 0 {
                    println!("Progress: {}", v);
                }
            }
        })
        .reduce(|a, b| match (a, b) {
            (None, None) => None,
            (Some(x), None) => Some(x),
            (None, Some(x)) => Some(x),
            (Some(mut a), Some(b)) => {
                for i in 0..N {
                    for j in 0..M {
                        (a.0)[i][j] += (b.0)[i][j];
                    }
                }
                a.1 += b.1;
                Some(a)
            }
        })
        .unwrap()
        .map(|a| (a.0, a.1)) // unboxes
        .unwrap_or_else(|| ([[0; M]; N], 0))
}
