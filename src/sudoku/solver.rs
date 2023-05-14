use serde::{Deserialize, Serialize};

use super::math::get_x_and_y_from_pos;
use super::{Notes, Sudoku};
use std::num::NonZeroU8;

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    #[default]
    Trivial,
    Easy,
    Medium,
    Advanced,
    Expert,
}

pub struct SolverResult {
    pub solution: Sudoku,
    pub difficulty: Difficulty,
}

/// Rates a Sudoku by difficulty level.
///
/// Returns `None` if the Sudoku cannot be solved.
pub fn rate_difficulty(sudoku: Sudoku) -> Option<Difficulty> {
    solve(sudoku).map(|result| result.difficulty)
}

/// Solves the Sudoku, if possible, and returns one of its solutions, along with
/// the rated difficulty.
pub fn solve(mut sudoku: Sudoku) -> Option<SolverResult> {
    let mut notes = Notes::from_sudoku(&sudoku);

    let mut difficulty = Difficulty::Trivial;
    'outer: while notes.has_notes() {
        // Fill in any places that only have a single number as the solution:
        for pos in 0..81 {
            if let Some(n) = notes.get_only_number(pos) {
                let (x, y) = get_x_and_y_from_pos(pos);
                sudoku = sudoku.set(x, y, n);
                notes.remove_all_notes_affected_by_set(x, y, n);
                continue 'outer;
            }
        }

        difficulty = std::cmp::max(difficulty, Difficulty::Easy);

        // Scan for lone rangers:
        for pos in 0..81 {
            if let Some(n) = notes.get_lone_ranger(pos) {
                let (x, y) = get_x_and_y_from_pos(pos);
                sudoku = sudoku.set(x, y, n);
                notes.remove_all_notes_affected_by_set(x, y, n);
                continue 'outer;
            }
        }

        difficulty = std::cmp::max(difficulty, Difficulty::Medium);

        // Scan for twins:
        for pos in 0..81 {
            if let Some(twins) = notes.find_twins(pos) {
                if notes.remove_all_notes_affected_by_twins(twins) {
                    continue 'outer;
                }
            }
        }

        // Scan for triplets:
        for pos in 0..81 {
            if let Some(triplets) = notes.find_triplets(pos) {
                if notes.remove_all_notes_affected_by_triplets(triplets) {
                    continue 'outer;
                }
            }
        }

        difficulty = std::cmp::max(difficulty, Difficulty::Advanced);

        // Scan for hidden twins:
        for pos in 0..81 {
            if let Some(twins) = notes.find_hidden_twins(pos) {
                if notes.remove_all_notes_affected_by_twins(twins) {
                    continue 'outer;
                }
            }
        }

        // Scan for hidden triplets:
        for pos in 0..81 {
            if let Some(triplets) = notes.find_hidden_triplets(pos) {
                if notes.remove_all_notes_affected_by_triplets(triplets) {
                    continue 'outer;
                }
            }
        }

        // Brute force is our last resort:
        return solve_through_brute_force(sudoku).map(|solution| SolverResult {
            solution,
            difficulty: Difficulty::Expert,
        });
    }

    sudoku.is_solved().then_some(SolverResult {
        solution: sudoku,
        difficulty,
    })
}

fn solve_through_brute_force(sudoku: Sudoku) -> Option<Sudoku> {
    // Search for a cell without any number:
    for y in 0..9 {
        for x in 0..9 {
            if sudoku.has(x, y) {
                continue; // Already filled in.
            }

            // Attempt to solve the Sudoku by recursively solving it
            // with 1 through 9 filled in the open cell:
            for n in 1..=9 {
                let n = NonZeroU8::new(n).unwrap();
                if sudoku.may_set(x, y, n) {
                    if let Some(result) = solve(sudoku.set(x, y, n)) {
                        return Some(result.solution);
                    }
                }
            }

            return None; // No solution could be found.
        }
    }

    // We didn't find any open cell, so it must already have been solved.
    Some(sudoku)
}
