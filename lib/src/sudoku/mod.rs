mod generator;
mod math;
mod notes;
mod persistence;
mod solver;

use bevy::prelude::Resource;
use solver::solve;
use std::fmt::{self, Write};
use std::num::NonZeroU8;
use std::ops::Sub;

pub use math::*;
pub use solver::Difficulty;

const START_MULTIPLIERS_BY_DIFFICULTY: [i32; 5] = [20, 40, 60, 80, 100];
const TIME_FOR_MULTIPLIER: i32 = 20;

pub struct SetNumberOptions {
    /// The current timer, in seconds.
    pub elapsed_secs: f32,

    /// Set to `true` if the number is being set as a hint.
    ///
    /// The user will not receive points for setting this number.
    pub is_hint: bool,

    /// Whether mistakes should be shown upfront. If `false`, the number will
    /// be filled in even if it is wrong. If `true`, and the number is wrong,
    /// it will be recorded in the game's `mistakes`.
    pub show_mistakes: bool,
}

/// A Sudoku game with a starting board and a solution, a current state, and
/// notes.
#[derive(Default, Resource)]
pub struct Game {
    pub start: Sudoku,
    pub solution: Sudoku,
    pub current: Sudoku,
    pub notes: Notes,
    pub mistakes: Notes,
    pub difficulty: Difficulty,
    pub score: u32,
    pub elapsed_secs: f32,
    pub num_mistakes: u32,
    pub num_hints: u32,
}

impl Game {
    /// Returns whether the game is in its default (uninitialized) state.
    pub fn is_default(&self) -> bool {
        self.start == Sudoku::default()
    }

    /// Returns whether all the instances of a given number have been filled in.
    pub fn is_completed(&self, n: NonZeroU8) -> bool {
        for pos in 0..81 {
            if let Some(solution_n) = self.solution.get_by_pos(pos) {
                if solution_n == n && self.current.get_by_pos(pos) != Some(n) {
                    return false;
                }
            }
        }

        true
    }

    /// Returns whether the game is (correctly) solved.
    pub fn is_solved(&self) -> bool {
        !self.is_default() && self.current == self.solution
    }

    /// Loads the tutorial game.
    pub fn load_tutorial() -> Self {
        Self {
            start: Sudoku::tutorial(),
            solution: solve(Sudoku::tutorial())
                .map(|result| result.solution)
                .expect("Cannot solve tutorial"),
            current: Sudoku::tutorial(),
            notes: Notes::default(),
            mistakes: Notes::default(),
            difficulty: Difficulty::Trivial,
            score: 0,
            elapsed_secs: 0.,
            num_mistakes: 0,
            num_hints: 1,
        }
    }

    /// Returns whether the game may be continued.
    pub fn may_continue(&self) -> bool {
        !self.is_default() && !self.is_solved()
    }

    /// Sets the given number `n` at the given `x` and `y` coordinates.
    ///
    /// Also increases the score or `num_mistakes`, depending on whether the
    /// number was correct.
    ///
    /// Returns `true` if the given number was correct, `false` otherwise.
    pub fn set(&mut self, x: u8, y: u8, n: NonZeroU8, options: SetNumberOptions) -> bool {
        let SetNumberOptions {
            elapsed_secs,
            is_hint,
            show_mistakes,
        } = options;

        if let Some(existing_n) = self.start.get(x, y) {
            // Starting numbers may not be replaced, but we can tell if they're right.
            return existing_n == n;
        }

        let is_correct = self.solution.get(x, y) == Some(n);
        if is_correct || !show_mistakes {
            self.current = self.current.set(x, y, n);
        }

        self.elapsed_secs = elapsed_secs;

        if !is_correct {
            self.num_mistakes += 1;
        }

        if is_correct && !is_hint {
            self.score += self.calculate_score(x, y, n) * self.calculate_multiplier();
        }

        if show_mistakes && !is_correct {
            self.mistakes.set(x, y, n);
            self.notes.unset(x, y, n);
        } else {
            self.notes.remove_all_notes_affected_by_set(x, y, n);
            self.mistakes.clear(x, y);
        }

        if cfg!(target_os = "ios") {
            self.save(); // Auto-save seems too unreliable otherwise.
        }

        is_correct
    }

    fn calculate_multiplier(&self) -> u32 {
        let mut multiplier_penalty = self.elapsed_secs as i32 / TIME_FOR_MULTIPLIER;

        // 1 mistake drops the multiplier by 2, 2 mistakes by 6, 3 mistakes by 12, etc..
        for i in 1..=self.num_mistakes {
            multiplier_penalty += 2 * i as i32;
        }

        // Similar strategy for hints, but with a less hefty penalty.
        for i in 1..=self.num_hints {
            multiplier_penalty += i as i32;
        }

        START_MULTIPLIERS_BY_DIFFICULTY[self.difficulty as usize]
            .sub(multiplier_penalty)
            .max(1) as u32
    }

    /// Calculates the score increase for setting the given number (without
    /// the multiplier). This assumes the number being set is correct.
    fn calculate_score(&self, x: u8, y: u8, n: NonZeroU8) -> u32 {
        let mut block_completed = true;
        let mut column_completed = true;
        let mut row_completed = true;
        let mut num_correct_positions_for_n = 0;

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

            // Check the row.
            if self.current.get(i, y) != self.solution.get(i, y) {
                row_completed = false;
            }

            // Check the number.
            for j in 0..9 {
                if self.current.get(i, j) == Some(n) && self.solution.get(i, j) == Some(n) {
                    num_correct_positions_for_n += 1;
                }
            }
        }

        let number_completed = num_correct_positions_for_n == 9;

        1 + if block_completed { 9 } else { 0 }
            + if column_completed { 9 } else { 0 }
            + if number_completed { 9 } else { 0 }
            + if row_completed { 9 } else { 0 }
    }

    /// Returns a hint, if any.
    ///
    /// If the user has made a mistake, it will be highlighted as a hint.
    /// Otherwise, it attempts to find the most obvious hint that will help the
    /// user get towards the solution.
    pub fn get_hint(&self) -> Option<Hint> {
        // First look for mistakes.
        for pos in 0..81 {
            if let Some(n) = self.current.get_by_pos(pos) {
                if self.solution.get_by_pos(pos) != Some(n) {
                    let (x, y) = get_x_and_y_from_pos(pos);
                    return Some(Hint { x, y });
                }
            }
        }

        let mut notes = Notes::from_sudoku(&self.current);
        'outer: while notes.has_notes() {
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
                        bevy::log::info!("Found twins: {twins:?}");
                        continue 'outer;
                    }
                }
            }

            // Find triplets:
            for pos in 0..81 {
                if let Some(triplets) = notes.find_triplets(pos) {
                    if notes.remove_all_notes_affected_by_triplets(triplets) {
                        bevy::log::info!("Found triplet: {triplets:?}");
                        continue 'outer;
                    }
                }
            }

            // Find hidden twins:
            for pos in 0..81 {
                if let Some(twins) = notes.find_hidden_twins(pos) {
                    if notes.remove_all_notes_affected_by_twins(twins) {
                        bevy::log::info!("Found hidden twin: {twins:?}");
                        continue 'outer;
                    }
                }
            }

            // Find hidden triplets:
            for pos in 0..81 {
                if let Some(triplets) = notes.find_hidden_triplets(pos) {
                    if notes.remove_all_notes_affected_by_triplets(triplets) {
                        bevy::log::info!("Found hidden triplet: {triplets:?}");
                        continue 'outer;
                    }
                }
            }

            // Screw it, just give some position with a note:
            for pos in 0..81 {
                if notes.has_some_number(pos) {
                    let hint = get_x_and_y_from_pos(pos).into();
                    bevy::log::info!("Random hint: {hint:?}");
                    return Some(hint);
                }
            }
        }

        None
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
        let mut cells = self.cells;
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
        let mut cells = self.cells;
        cells[pos] = None;
        Self { cells }
    }

    pub const fn tutorial() -> Self {
        Self {
            cells: unsafe {
                [
                    None,
                    Some(NonZeroU8::new_unchecked(2)),
                    None,
                    None,
                    Some(NonZeroU8::new_unchecked(7)),
                    None,
                    Some(NonZeroU8::new_unchecked(9)),
                    Some(NonZeroU8::new_unchecked(1)),
                    None,
                    None,
                    None,
                    Some(NonZeroU8::new_unchecked(9)),
                    Some(NonZeroU8::new_unchecked(1)),
                    None,
                    None,
                    Some(NonZeroU8::new_unchecked(4)),
                    None,
                    Some(NonZeroU8::new_unchecked(7)),
                    None,
                    Some(NonZeroU8::new_unchecked(7)),
                    Some(NonZeroU8::new_unchecked(1)),
                    None,
                    Some(NonZeroU8::new_unchecked(5)),
                    None,
                    Some(NonZeroU8::new_unchecked(8)),
                    None,
                    None,
                    Some(NonZeroU8::new_unchecked(8)),
                    None,
                    Some(NonZeroU8::new_unchecked(2)),
                    Some(NonZeroU8::new_unchecked(5)),
                    None,
                    Some(NonZeroU8::new_unchecked(1)),
                    None,
                    None,
                    None,
                    Some(NonZeroU8::new_unchecked(9)),
                    None,
                    Some(NonZeroU8::new_unchecked(3)),
                    None,
                    None,
                    None,
                    None,
                    Some(NonZeroU8::new_unchecked(4)),
                    Some(NonZeroU8::new_unchecked(5)),
                    Some(NonZeroU8::new_unchecked(4)),
                    Some(NonZeroU8::new_unchecked(5)),
                    None,
                    None,
                    Some(NonZeroU8::new_unchecked(2)),
                    None,
                    None,
                    None,
                    Some(NonZeroU8::new_unchecked(1)),
                    Some(NonZeroU8::new_unchecked(7)),
                    None,
                    None,
                    Some(NonZeroU8::new_unchecked(8)),
                    None,
                    Some(NonZeroU8::new_unchecked(4)),
                    None,
                    Some(NonZeroU8::new_unchecked(2)),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                    Some(NonZeroU8::new_unchecked(7)),
                    Some(NonZeroU8::new_unchecked(5)),
                    Some(NonZeroU8::new_unchecked(9)),
                    Some(NonZeroU8::new_unchecked(4)),
                    None,
                    Some(NonZeroU8::new_unchecked(4)),
                    None,
                    Some(NonZeroU8::new_unchecked(3)),
                    Some(NonZeroU8::new_unchecked(9)),
                    Some(NonZeroU8::new_unchecked(5)),
                    None,
                    None,
                    None,
                ]
            },
        }
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

#[derive(Debug)]
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
