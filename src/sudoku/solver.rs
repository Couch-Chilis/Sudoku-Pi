use super::math::get_x_and_y_from_pos;
use super::notes::Notes;
use super::Sudoku;
use std::num::NonZeroU8;

/// Rates a Sudoku by difficulty level.
///
/// Returns `None` if the Sudoku cannot be solved.
pub fn rate_difficulty(sudoku: Sudoku) -> Option<u8> {
    solve_intelligently(sudoku).map(|result| result.difficulty)
}

/// Solves the Sudoku, if possible, and returns the solutions.
pub fn solve(sudoku: Sudoku) -> Option<Sudoku> {
    solve_intelligently(sudoku).map(|result| result.solution)
}

struct SolverResult {
    solution: Sudoku,
    difficulty: u8,
}

fn solve_intelligently(mut sudoku: Sudoku) -> Option<SolverResult> {
    let mut notes = Notes::from_sudoku(&sudoku);

    let mut difficulty = 0;
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

        difficulty = std::cmp::max(difficulty, 1);

        // Scan for lone rangers:
        for pos in 0..81 {
            if let Some(n) = notes.get_lone_ranger(pos) {
                let (x, y) = get_x_and_y_from_pos(pos);
                sudoku = sudoku.set(x, y, n);
                notes.remove_all_notes_affected_by_set(x, y, n);
                continue 'outer;
            }
        }

        difficulty = std::cmp::max(difficulty, 2);

        // Scan for twins:
        for pos in 0..81 {
            if let Some(twins) = notes.find_twins(pos) {
                if notes.remove_all_notes_affected_by_twins(twins) {
                    continue 'outer;
                }
            }
        }

        difficulty = std::cmp::max(difficulty, 3);

        // Scan for triplets:
        for pos in 0..81 {
            if let Some(triplets) = notes.find_triplets(pos) {
                if notes.remove_all_notes_affected_by_triplets(triplets) {
                    continue 'outer;
                }
            }
        }

        // Brute force is our last resort:
        return solve_through_brute_force(sudoku).map(|solution| SolverResult {
            solution,
            difficulty: 4,
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
                    if let Some(result) = solve_intelligently(sudoku.set(x, y, n)) {
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
