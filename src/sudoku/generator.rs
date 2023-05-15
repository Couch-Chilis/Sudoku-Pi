use super::math::get_x_and_y_from_pos;
use super::solver::{rate_difficulty, solve, Difficulty};
use super::Sudoku;
use crate::sudoku::math::get_pos;
use crate::sudoku::solver::SolverResult;
use anyhow::{bail, Context};
use rand::seq::SliceRandom;
use rand::Rng;
use std::num::NonZeroU8;

impl super::Game {
    /// Generates a new game at the given difficulty level.
    pub fn generate(difficulty: Difficulty) -> anyhow::Result<Self> {
        let mut num_tries = 0;
        loop {
            let start = generate_sudoku(difficulty)?;

            let SolverResult {
                solution,
                difficulty: rated_difficulty,
            } = solve(start.clone())
                .context("Blimey, I could not even solve my own generated Sudoku...")?;
            bevy::log::info!(
                "Generated Sudoku of difficulty {rated_difficulty:?} (requested: {difficulty:?})"
            );

            num_tries += 1;
            if rated_difficulty == difficulty || num_tries == 3 {
                return Ok(Self {
                    solution,
                    start: start.clone(),
                    current: start,
                    notes: Default::default(),
                    difficulty,
                    score: 0,
                    elapsed_secs: 0.,
                });
            }
        }
    }
}

fn generate_sudoku(difficulty: Difficulty) -> anyhow::Result<Sudoku> {
    let Some(solution) = create_solution(Sudoku::new(), 0, 0) else {
        bail!("Oh boy, I could not even create a solution...");
    };

    let DiggingStrategy {
        digging_order,
        min_numbers_per_line,
        num_cells_to_dig,
        num_cells_for_most_difficult_number,
    } = DiggingStrategy::generate(difficulty)?;

    let mut start = solution;
    let mut num_cells_left_for_most_difficult_number = 9;
    let most_difficult_number = NonZeroU8::new(rand::thread_rng().gen_range(1..10)).unwrap();
    for i in 0..81 {
        let pos = digging_order[80 - i]; // Dig in reverse order.
        if start.get_by_pos(pos) == Some(most_difficult_number) {
            start = start.unset_by_pos(pos);
            num_cells_left_for_most_difficult_number -= 1;
            if num_cells_left_for_most_difficult_number == num_cells_for_most_difficult_number {
                break;
            }
        }
    }

    let mut num_cells_dug = 0;
    'dig: for i in digging_order {
        let (x, y) = get_x_and_y_from_pos(i);

        if min_numbers_per_line > 0 {
            let mut num_others_in_column = 0;
            let mut num_others_in_row = 0;
            for j in 0..9 {
                if j != y && start.has(x, j) {
                    num_others_in_column += 1;
                }
                if j != x && start.has(j, y) {
                    num_others_in_row += 1;
                }
            }

            if num_others_in_column < min_numbers_per_line
                || num_others_in_row < min_numbers_per_line
            {
                // We would be left with too few numbers in a single row or
                // column, so continue before we let that happen.
                continue 'dig;
            }
        }

        let n = start.get(x, y);

        // Determine whether the Sudoku remains unique after digging the
        // number by trying whether the Sudoku is solvable with any other
        // number filled in at the cell.
        for other_n in 1..=9 {
            let other_n = NonZeroU8::new(other_n).unwrap();
            if Some(other_n) != n
                && start.may_set(x, y, other_n)
                && solve(start.set(x, y, other_n)).is_some()
            {
                continue 'dig; // It wouldn't remain unique otherwise.
            }
        }

        let new_start = start.unset(x, y);
        if difficulty < Difficulty::Expert {
            let rated_difficulty = rate_difficulty(new_start.clone())
                .context("Yikes, I could not even rate my own starting position...")?;
            if rated_difficulty > difficulty {
                continue 'dig; // It would become too difficult otherwise.
            }
        }

        start = new_start;
        num_cells_dug += 1;
        if num_cells_dug >= num_cells_to_dig {
            break;
        }
    }

    Ok(start)
}

/// Attempt to create a solution by recursively filling the cells, starting at a
/// random number to create unique solutions.
///
/// Returns a random valid solution if one exists.
fn create_solution(sudoku: Sudoku, mut x: u8, mut y: u8) -> Option<Sudoku> {
    if x > 8 {
        y += 1;
        if y > 8 {
            return Some(sudoku); // Finito!
        }

        x = 0;
    }

    let offset = rand::thread_rng().gen_range(0..9);
    for i in 0..9 {
        let n = NonZeroU8::new((i + offset) % 9 + 1).unwrap();
        if !sudoku.may_set(x, y, n) {
            continue;
        }

        if let Some(solution) = create_solution(sudoku.set(x, y, n), x + 1, y) {
            return Some(solution);
        }
    }

    // No solution could be found.
    None
}

/// Represents the digging strategy to create a Sudoku board that maintains a
/// unique solution.
struct DiggingStrategy {
    /// The order in which to attempt digging of cells. Only cells that can be
    /// cleared while maintaining a unique solution are actually "dug".
    digging_order: [usize; 81],

    /// The minimal amount of numbers that should be left on each line.
    min_numbers_per_line: u8,

    /// The amount of cells that should be dug before we consider the board
    /// difficult enough.
    num_cells_to_dig: u8,

    /// Before the general digging starts, we dig one number across the board
    /// to make it more difficult than others. This setting defines the amount
    /// of cells to leave undug for that number.
    num_cells_for_most_difficult_number: u8,
}

impl DiggingStrategy {
    /// Generates a digging strategy to be used for the given difficulty level.
    pub fn generate(difficulty: Difficulty) -> anyhow::Result<Self> {
        match difficulty {
            Difficulty::Trivial => Ok(Self {
                digging_order: get_random_digging_order(),
                min_numbers_per_line: 5,
                num_cells_to_dig: 31,
                num_cells_for_most_difficult_number: 3,
            }),
            Difficulty::Easy => Ok(Self {
                digging_order: get_random_digging_order(),
                min_numbers_per_line: 4,
                num_cells_to_dig: 45,
                num_cells_for_most_difficult_number: 2,
            }),
            Difficulty::Medium => Ok(Self {
                digging_order: get_checkered_digging_order(),
                min_numbers_per_line: 3,
                num_cells_to_dig: 50,
                num_cells_for_most_difficult_number: 1,
            }),
            Difficulty::Advanced => Ok(Self {
                digging_order: get_linear_digging_order(),
                min_numbers_per_line: 2,
                num_cells_to_dig: 54,
                num_cells_for_most_difficult_number: 0,
            }),
            Difficulty::Expert => Ok(Self {
                digging_order: get_spiraling_digging_order(),
                min_numbers_per_line: 0,
                num_cells_to_dig: 59,
                num_cells_for_most_difficult_number: 0,
            }),
        }
    }
}

fn get_checkered_digging_order() -> [usize; 81] {
    let digging_order: Vec<usize> = (0..41)
        .map(|n| 2 * n)
        .chain((0..40).map(|n| 2 * n + 1))
        .collect();
    digging_order.try_into().unwrap()
}

fn get_linear_digging_order() -> [usize; 81] {
    let digging_order: Vec<usize> = if rand::thread_rng().gen_bool(0.5) {
        (0..81).rev().collect()
    } else {
        (0..81).collect()
    };
    digging_order.try_into().unwrap()
}

fn get_random_digging_order() -> [usize; 81] {
    let mut digging_order: Vec<usize> = (0..81).collect();
    digging_order.shuffle(&mut rand::thread_rng());
    digging_order.try_into().unwrap()
}

fn get_spiraling_digging_order() -> [usize; 81] {
    enum Direction {
        Up,
        Right,
        Down,
        Left,
    }

    let (mut x, mut y, mut direction) = match rand::thread_rng().gen_range(0..4) {
        0 => (0, 0, Direction::Right),
        1 => (8, 0, Direction::Down),
        2 => (8, 8, Direction::Left),
        _ => (0, 8, Direction::Up),
    };

    let mut digging_order = [0; 81];
    for i in 0..81 {
        digging_order[i] = get_pos(x, y);

        match direction {
            Direction::Right => {
                if x == 8 || digging_order[0..i].contains(&get_pos(x + 1, y)) {
                    y += 1;
                    direction = Direction::Down;
                } else {
                    x += 1;
                }
            }
            Direction::Down => {
                if y == 8 || digging_order[0..i].contains(&get_pos(x, y + 1)) {
                    x -= 1;
                    direction = Direction::Left;
                } else {
                    y += 1;
                }
            }
            Direction::Left => {
                if x == 0 || digging_order[0..i].contains(&get_pos(x - 1, y)) {
                    y -= 1;
                    direction = Direction::Up;
                } else {
                    x -= 1;
                }
            }
            Direction::Up => {
                if y == 0 || digging_order[0..i].contains(&get_pos(x, y - 1)) {
                    x += 1;
                    direction = Direction::Right;
                } else {
                    y -= 1;
                }
            }
        }
    }

    digging_order
}
