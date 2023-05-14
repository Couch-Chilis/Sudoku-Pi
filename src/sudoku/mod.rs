mod generator;
mod math;
mod notes;
mod persistence;
mod solver;

use bevy::prelude::Resource;
use solver::solve;
use std::fmt::{self, Write};
use std::num::NonZeroU8;

pub use math::*;

const START_MULTIPLIERS_BY_DIFFICULTY: [i32; 5] = [20, 40, 60, 80, 100];
const TIME_FOR_MULTIPLIER: i32 = 20;

/// A Sudoku game with a starting board and a solution, a current state, and
/// notes.
#[derive(Default, Resource)]
pub struct Game {
    pub start: Sudoku,
    pub solution: Sudoku,
    pub current: Sudoku,
    pub notes: Notes,
    pub difficulty: u8,
    pub score: u32,
    pub elapsed_secs: f32,
}

impl Game {
    /// Returns whether the game is in its default (uninitialized) state.
    pub fn is_default(&self) -> bool {
        self.start == Sudoku::default()
    }

    /// Returns whether the game is (correctly) solved.
    pub fn is_solved(&self) -> bool {
        !self.is_default() && self.current == self.solution
    }

    /// Returns whether the game may be continued.
    pub fn may_continue(&self) -> bool {
        !self.is_default() && !self.is_solved()
    }

    /// Sets the given number `n` at the given `x` and `y` coordinates.
    ///
    /// The given `elapsed_secs` are used to calculate the increase in score if
    /// the correct number is given.
    ///
    /// Returns the `elapsed_secs` with a possible penalty applied if the set
    /// number was incorrect.
    pub fn set(&mut self, x: u8, y: u8, n: NonZeroU8, elapsed_secs: f32) -> f32 {
        if self.start.has(x, y) {
            return elapsed_secs; // Starting numbers may not be replaced.
        }

        // We need an updated time spent to calculate the score increase.
        self.elapsed_secs = elapsed_secs;

        self.current = self.current.set(x, y, n);
        self.notes.remove_all_notes_affected_by_set(x, y, n);

        if self.solution.get(x, y) == Some(n) {
            let multiplier = (START_MULTIPLIERS_BY_DIFFICULTY
                .get(self.difficulty as usize)
                .cloned()
                .unwrap_or_default()
                - elapsed_secs as i32 / TIME_FOR_MULTIPLIER)
                .max(1);
            self.score += self.calculate_score(x, y, n) * multiplier as u32;

            elapsed_secs
        } else {
            elapsed_secs + TIME_FOR_MULTIPLIER as f32
        }
    }

    /// Calculates the score increase for setting the given number (without
    /// the multiplier). This assumes the number being set is correct.
    fn calculate_score(&self, x: u8, y: u8, n: NonZeroU8) -> u32 {
        let mut block_completed = true;
        let mut column_completed = true;
        let mut number_completed = true;
        let mut row_completed = true;

        let block_offset_x = get_block_offset(x);
        let block_offset_y = get_block_offset(y);
        for i in 0..9 {
            // Check the block.
            let block_x = block_offset_x + i % 3;
            let block_y = block_offset_y + i / 3;
            if self.current.get(block_x, block_y) != self.solution.get(block_x, block_y) {
                block_completed = false;
            }

            // Check the column.
            if self.current.get(x, i) != self.solution.get(x, i) {
                column_completed = false;
            }

            // Check the number.
            let mut n_found = false;
            for j in 0..9 {
                if self.current.get(i, j) == Some(n) && self.solution.get(i, j) == Some(n) {
                    n_found = true;
                }
            }
            if !n_found {
                number_completed = false;
            }

            // Check the row.
            if self.current.get(i, y) != self.solution.get(i, y) {
                row_completed = false;
            }
        }

        1 + if block_completed { 9 } else { 0 }
            + if column_completed { 9 } else { 0 }
            + if number_completed { 9 } else { 0 }
            + if row_completed { 9 } else { 0 }
    }
}

/// Keeps track of all the cells within the Sudoku board.
#[derive(Clone, PartialEq)]
pub struct Sudoku {
    cells: [Cell; 81],
}

impl Sudoku {
    /// Creates a new, empty Sudoku without any of the cells filled in.
    pub fn new() -> Self {
        Self { cells: [None; 81] }
    }

    /// Returns the only unique solution to this Sudoku.
    ///
    /// Returns `None` if there are multiple solutions.
    pub fn find_unique_solution(&self) -> Option<Self> {
        let mut sudoku = self.clone();
        for y in 0..9 {
            for x in 0..9 {
                if sudoku.has(x, y) {
                    continue;
                }

                let mut number: Option<NonZeroU8> = None;
                for n in 1..=9 {
                    let n = NonZeroU8::new(n).unwrap();
                    if self.may_set(x, y, n) && solve(sudoku.set(x, y, n)).is_some() {
                        if number.is_some() {
                            return None; // Only a single number is allowed.
                        } else {
                            number = Some(n);
                        }
                    }
                }

                if let Some(n) = number {
                    // Filling in the numbers that are known speeds up
                    // follow-up calls to `solve()`.
                    sudoku = sudoku.set(x, y, n);
                }
            }
        }

        Some(sudoku)
    }

    /// Returns the value of the cell at the given coordinates.
    pub fn get(&self, x: u8, y: u8) -> Cell {
        self.cells[get_pos(x, y)]
    }

    /// Returns the value of the cell with the given position.
    #[inline]
    pub fn get_by_pos(&self, pos: usize) -> Cell {
        self.cells[pos]
    }

    /// Returns a hint, if any.
    ///
    /// This function attempts to find the most easy hint that will help the
    /// user get towards the solution before
    pub fn get_hint(&self) -> Option<Hint> {
        let mut notes = Notes::from_sudoku(self);
        if !notes.has_notes() {
            return None;
        }

        // Find a place that only has a single number:
        for pos in 0..81 {
            if notes.get_only_number(pos).is_some() {
                return Some(get_x_and_y_from_pos(pos).into());
            }
        }

        // Find a lone ranger:
        for pos in 0..81 {
            if notes.get_lone_ranger(pos).is_some() {
                return Some(get_x_and_y_from_pos(pos).into());
            }
        }

        // Find twins:
        for pos in 0..81 {
            if let Some(twins) = notes.find_twins(pos) {
                if notes.remove_all_notes_affected_by_twins(twins) {
                    return Some(get_x_and_y_from_pos(pos).into());
                }
            }
        }

        // Find triplets:
        for pos in 0..81 {
            if let Some(triplets) = notes.find_triplets(pos) {
                if notes.remove_all_notes_affected_by_triplets(triplets) {
                    return Some(get_x_and_y_from_pos(pos).into());
                }
            }
        }

        // Screw it, just give some position with a note:
        for pos in 0..81 {
            if notes.has_some_number(pos) {
                return Some(get_x_and_y_from_pos(pos).into());
            }
        }

        None
    }

    /// Returns whether the cell at the given coordinates has a number.
    pub fn has(&self, x: u8, y: u8) -> bool {
        self.cells[get_pos(x, y)].is_some()
    }

    /// Returns whether the Sudoku is (correctly) solved.
    pub fn is_solved(&self) -> bool {
        for y in 0..9 {
            for x in 0..9 {
                if let Some(n) = self.get(x, y) {
                    if !self.may_set(x, y, n) {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }

        true
    }

    /// Returns whether the given number may be filled in in the cell with the
    /// given coordinates.
    pub fn may_set(&self, x: u8, y: u8, n: NonZeroU8) -> bool {
        let block_offset_x = get_block_offset(x);
        let block_offset_y = get_block_offset(y);
        for i in 0..9 {
            // Check the row.
            if i != x && self.get(i, y) == Some(n) {
                return false;
            }

            // Check the column.
            if i != y && self.get(x, i) == Some(n) {
                return false;
            }

            // Check the block.
            let block_x = block_offset_x + i % 3;
            let block_y = block_offset_y + i / 3;
            if (block_x != x || block_y != y) && self.get(block_x, block_y) == Some(n) {
                return false;
            }
        }

        true
    }

    /// Returns a new Sudoku board with the given number filled in at the given
    /// coordinates.
    pub fn set(&self, x: u8, y: u8, n: NonZeroU8) -> Self {
        let mut cells = self.cells.clone();
        cells[get_pos(x, y)] = Some(n);
        Self { cells }
    }

    /// Returns a new Sudoku board with the number cleared (or "dug") from the
    /// cell at the given coordinates.
    pub fn unset(&self, x: u8, y: u8) -> Self {
        self.unset_by_pos(get_pos(x, y))
    }

    /// Returns a new Sudoku board with the number cleared (or "dug") from the
    /// cell at the given position.
    #[inline]
    pub fn unset_by_pos(&self, pos: usize) -> Self {
        let mut cells = self.cells.clone();
        cells[pos] = None;
        Self { cells }
    }
}

impl Default for Sudoku {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Sudoku {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..9 {
            for x in 0..9 {
                let c = self.cells[get_pos(x, y)]
                    .and_then(|n| char::from_digit(n.get() as u32, 10))
                    .unwrap_or('0');
                f.write_char(c)?;

                if x == 8 {
                    if y < 8 {
                        f.write_char('\n')?;
                    }
                } else {
                    f.write_char(' ')?;
                }
            }
        }

        Ok(())
    }
}

/// A single cell within the Sudoku board, which may or may not have a number.
pub type Cell = Option<NonZeroU8>;

pub struct Hint {
    pub x: u8,
    pub y: u8,
}

impl From<(u8, u8)> for Hint {
    fn from((x, y): (u8, u8)) -> Self {
        Self { x, y }
    }
}

/// Keeps track of notes within the Sudoku board.
///
/// Every cell can have 9 notes, which are represented using bit flags encoded
/// in `u16` fields.
#[derive(Clone)]
pub struct Notes {
    cells: [u16; 81],
}
