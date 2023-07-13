use super::math::{get_block_offset, get_pos, get_x_and_y_from_pos};
use super::{Notes, Sudoku};
use std::num::NonZeroU8;

#[derive(Clone, Copy, Debug)]
pub struct Twins {
    pub x1: u8,
    pub y1: u8,
    pub x2: u8,
    pub y2: u8,
    pub twin_notes: u16,
}

#[derive(Clone, Copy, Debug)]
pub struct Triplets {
    pub x1: u8,
    pub y1: u8,
    pub x2: u8,
    pub y2: u8,
    pub x3: u8,
    pub y3: u8,
    pub triplet_notes: u16,
}

impl Notes {
    /// Returns a new, empty set of notes.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a new set of notes based on the state of the given Sudoku.
    ///
    /// Notes are initialized such that every number which may be filled into a
    /// cell without conflicts will be a part of that cell's notes.
    pub fn from_sudoku(sudoku: &Sudoku) -> Self {
        let mut notes = Notes::new();
        for y in 0..9 {
            for x in 0..9 {
                if !sudoku.has(x, y) {
                    for n in 1..=9 {
                        let n = NonZeroU8::new(n).unwrap();
                        if sudoku.may_set(x, y, n) {
                            notes.set(x, y, n);
                        }
                    }
                }
            }
        }

        notes
    }

    /// Clears the notes in a single cell.
    pub fn clear(&mut self, x: u8, y: u8) {
        self.cells[get_pos(x, y)] = 0;
    }

    /// A set of twins are two cells that are within range of one another that
    /// both have the same numbers in their notes.
    ///
    /// This function checks if the cell at the given position is one of a set
    /// of twins and returns the coordinates for the twins if so.
    ///
    /// Assumes that all the notes are correctly filled in.
    pub fn find_twins(&self, pos: usize) -> Option<Twins> {
        let twin_notes = self.cells[pos];
        if get_num_notes(twin_notes) != 2 {
            return None;
        }

        let (x1, y1) = get_x_and_y_from_pos(pos);

        let block_offset_x = get_block_offset(x1);
        let block_offset_y = get_block_offset(y1);
        for i in 0..9 {
            // Check the row.
            if i != x1 && self.get(i, y1) == twin_notes {
                return Some(Twins {
                    x1,
                    y1,
                    x2: i,
                    y2: y1,
                    twin_notes,
                });
            }

            // Check the column.
            if i != y1 && self.get(x1, i) == twin_notes {
                return Some(Twins {
                    x1,
                    y1,
                    x2: x1,
                    y2: i,
                    twin_notes,
                });
            }

            // Check the block.
            let block_x = block_offset_x + i % 3;
            let block_y = block_offset_y + i / 3;
            if (block_x != x1 || block_y != y1) && self.get(block_x, block_y) == twin_notes {
                return Some(Twins {
                    x1,
                    y1,
                    x2: block_x,
                    y2: block_y,
                    twin_notes,
                });
            }
        }

        None
    }

    /// A set of triplets are three cells that are within range of one another
    /// that all have the same numbers in their notes.
    ///
    /// This function checks if the cell at the given position is one of a set
    /// of triplets and returns the coordinates for the triplets if so.
    ///
    /// Assumes that all the notes are correctly filled in.
    pub fn find_triplets(&self, pos: usize) -> Option<Triplets> {
        let triplet_notes = self.cells[pos];
        if get_num_notes(triplet_notes) != 3 {
            return None;
        }

        let (x1, y1) = get_x_and_y_from_pos(pos);

        let block_offset_x = get_block_offset(x1);
        let block_offset_y = get_block_offset(y1);
        for i in 0..9 {
            for j in 0..9 {
                if j == i {
                    continue;
                }

                // Check the row.
                if i != x1
                    && j != x1
                    && self.get(i, y1) == triplet_notes
                    && self.get(j, y1) == triplet_notes
                {
                    return Some(Triplets {
                        x1,
                        y1,
                        x2: i,
                        y2: y1,
                        x3: j,
                        y3: y1,
                        triplet_notes,
                    });
                }

                // Check the column.
                if i != y1
                    && j != y1
                    && self.get(x1, i) == triplet_notes
                    && self.get(x1, j) == triplet_notes
                {
                    return Some(Triplets {
                        x1,
                        y1,
                        x2: x1,
                        y2: i,
                        x3: x1,
                        y3: j,
                        triplet_notes,
                    });
                }

                // Check the block.
                let block_x1 = block_offset_x + i % 3;
                let block_y1 = block_offset_y + i / 3;
                let block_x2 = block_offset_x + j % 3;
                let block_y2 = block_offset_y + j / 3;
                if (block_x1 != x1 || block_y1 != y1)
                    && (block_x2 != x1 || block_y2 != y1)
                    && self.get(block_x1, block_y1) == triplet_notes
                    && self.get(block_x2, block_y2) == triplet_notes
                {
                    return Some(Triplets {
                        x1,
                        y1,
                        x2: block_x1,
                        y2: block_y1,
                        x3: block_x2,
                        y3: block_y2,
                        triplet_notes,
                    });
                }
            }
        }

        None
    }

    /// Hidden twins are like twins in that they refer to two cells that share
    /// two notes, but they are harder to find due to other numbers sharing
    /// their cells. The key to finding hidden twins is realizing there are no
    /// other cells in the same row, column, or block where the two shared
    /// numbers are present.
    ///
    /// This function checks if the cell at the given position is one of a set
    /// of hidden twins and returns the coordinates for the twins if so. This
    /// will not find regular twins.
    ///
    /// Assumes that all the notes are correctly filled in.
    pub fn find_hidden_twins(&self, pos: usize) -> Option<Twins> {
        let cell_notes = self.cells[pos];
        if get_num_notes(cell_notes) <= 2 {
            return None;
        }

        let (x1, y1) = get_x_and_y_from_pos(pos);

        let block_offset_x = get_block_offset(x1);
        let block_offset_y = get_block_offset(y1);
        for twin_notes in get_twin_permutations(cell_notes) {
            // Check the row.
            if let Some(x2) = find_hidden_twin(twin_notes, |i| (i != x1).then(|| self.get(i, y1))) {
                return Some(Twins {
                    x1,
                    y1,
                    x2,
                    y2: y1,
                    twin_notes,
                });
            }

            // Check the column.
            if let Some(y2) = find_hidden_twin(twin_notes, |i| (i != y1).then(|| self.get(x1, i))) {
                return Some(Twins {
                    x1,
                    y1,
                    x2: x1,
                    y2,
                    twin_notes,
                });
            }

            // Check the block.
            if let Some(i) = find_hidden_twin(twin_notes, |i| {
                let block_x = block_offset_x + i % 3;
                let block_y = block_offset_y + i / 3;
                (block_x != x1 || block_y != y1).then(|| self.get(block_x, block_y))
            }) {
                return Some(Twins {
                    x1,
                    y1,
                    x2: block_offset_x + i % 3,
                    y2: block_offset_y + i / 3,
                    twin_notes,
                });
            }
        }

        None
    }

    /// Hidden triplets are like triplets in that they refer to three cells that
    /// collectively share three notes, but they are harder to find due to one
    /// of two possible reaons:
    /// * Either they share numbers that may also be in other cells.
    /// * And/or the numbers they share are distributed unevenly amongst them.
    ///
    /// The key to finding hidden triplets is realizing there are no other cells
    /// in the same row, column, or block where the three numbers shared among
    /// the triplets may be present.
    ///
    /// This function checks if the cell at the given position is one of a set
    /// of triplets and returns the coordinates for the triplets if so.
    ///
    /// Assumes that all the notes are correctly filled in.
    pub fn find_hidden_triplets(&self, pos: usize) -> Option<Triplets> {
        let cell_notes = self.cells[pos];
        if get_num_notes(cell_notes) < 2 {
            return None;
        }

        let (x1, y1) = get_x_and_y_from_pos(pos);

        // Check the row.
        let notes_in_row = collect_notes_in_range(|i| self.get(i, y1));
        for triplet_notes in get_triplet_permutations(notes_in_row) {
            if let Some((x2, x3)) =
                find_hidden_triplets(triplet_notes, |i| (i != x1).then(|| self.get(i, y1)))
            {
                return Some(Triplets {
                    x1,
                    y1,
                    x2,
                    y2: y1,
                    x3,
                    y3: y1,
                    triplet_notes,
                });
            }
        }

        // Check the column.
        let notes_in_column = collect_notes_in_range(|i| self.get(x1, i));
        for triplet_notes in get_triplet_permutations(notes_in_column) {
            if let Some((y2, y3)) =
                find_hidden_triplets(triplet_notes, |i| (i != y1).then(|| self.get(x1, i)))
            {
                return Some(Triplets {
                    x1,
                    y1,
                    x2: x1,
                    y2,
                    x3: x1,
                    y3,
                    triplet_notes,
                });
            }
        }

        // Check the block.
        let block_offset_x = get_block_offset(x1);
        let block_offset_y = get_block_offset(y1);
        let notes_in_block = collect_notes_in_range(|i| {
            let block_x = block_offset_x + i % 3;
            let block_y = block_offset_y + i / 3;
            self.get(block_x, block_y)
        });
        for triplet_notes in get_triplet_permutations(notes_in_block) {
            if let Some((i, j)) = find_hidden_triplets(triplet_notes, |i| {
                let block_x = block_offset_x + i % 3;
                let block_y = block_offset_y + i / 3;
                (block_x != x1 || block_y != y1).then(|| self.get(block_x, block_y))
            }) {
                return Some(Triplets {
                    x1,
                    y1,
                    x2: block_offset_x + i % 3,
                    y2: block_offset_y + i / 3,
                    x3: block_offset_x + j % 3,
                    y3: block_offset_y + j / 3,
                    triplet_notes,
                });
            }
        }

        None
    }

    /// Returns the raw notes for the cell with the given coordinates.
    #[inline]
    fn get(&self, x: u8, y: u8) -> u16 {
        self.cells[get_pos(x, y)]
    }

    /// Returns all the notes that were present in `other` which are not present
    /// in the current notes.
    pub fn get_cleared_since(&self, other: &Self) -> Vec<(u8, u8, NonZeroU8)> {
        let mut cleared_notes = Vec::new();
        for pos in 0..81 {
            let current_cell = self.cells[pos];
            let other_cell = other.cells[pos];

            if current_cell != other_cell {
                for n in 1..9 {
                    let bit = 1 << n;
                    if other_cell & bit > 0 && current_cell & bit == 0 {
                        let (x, y) = get_x_and_y_from_pos(pos);
                        cleared_notes.push((x, y, NonZeroU8::new(n).unwrap()));
                    }
                }
            }
        }

        cleared_notes
    }

    /// The lone ranger is Sudoku's most basic strategy for finding numbers to
    /// fill in: When a cell is the only cell within a row, column, or block
    /// that is a valid position for a given number, that number must be the
    /// one to be filled in.
    ///
    /// This function checks if the notes for the cell at the given position
    /// contain a lone ranger and returns its number if so.
    ///
    /// Assumes that all the notes are correctly filled in.
    pub fn get_lone_ranger(&self, pos: usize) -> Option<NonZeroU8> {
        let (x, y) = get_x_and_y_from_pos(pos);

        let block_offset_x = get_block_offset(x);
        let block_offset_y = get_block_offset(y);

        for n in 1..=9 {
            let shifted_n = 1 << n;
            if self.cells[pos] & shifted_n == 0 {
                continue;
            }

            let mut row_busted = false;
            let mut column_busted = false;
            let mut block_busted = false;
            for i in 0..9 {
                // Check the row.
                if !row_busted && i != x && self.get(i, y) & shifted_n != 0 {
                    row_busted = true;
                }

                // Check the column.
                if !column_busted && i != y && self.get(x, i) & shifted_n != 0 {
                    column_busted = true;
                }

                // Check the block.
                if !block_busted {
                    let block_x = block_offset_x + i % 3;
                    let block_y = block_offset_y + i / 3;
                    if (block_x != x || block_y != y) && self.get(block_x, block_y) & shifted_n != 0
                    {
                        block_busted = true;
                    }
                }
            }

            if row_busted && column_busted && block_busted {
                continue;
            }

            return Some(NonZeroU8::new(n).unwrap());
        }

        None
    }

    /// Returns the only number that is present in the notes for the cell at the
    /// given position. Returns `None` if there are more or less than exactly 1
    /// number present in the cell.
    pub fn get_only_number(&self, pos: usize) -> Option<NonZeroU8> {
        let cell = self.cells[pos];
        match cell {
            2 => Some(NonZeroU8::new(1).unwrap()),
            4 => Some(NonZeroU8::new(2).unwrap()),
            8 => Some(NonZeroU8::new(3).unwrap()),
            16 => Some(NonZeroU8::new(4).unwrap()),
            32 => Some(NonZeroU8::new(5).unwrap()),
            64 => Some(NonZeroU8::new(6).unwrap()),
            128 => Some(NonZeroU8::new(7).unwrap()),
            256 => Some(NonZeroU8::new(8).unwrap()),
            512 => Some(NonZeroU8::new(9).unwrap()),
            _ => None,
        }
    }

    /// Returns whether the notes for the cell at the given position contain the
    /// given number.
    pub fn has(&self, x: u8, y: u8, n: NonZeroU8) -> bool {
        let val = 1 << n.get();
        self.get(x, y) & val == val
    }

    /// Returns whether any notes are present at all.
    pub fn has_notes(&self) -> bool {
        for pos in 0..81 {
            if self.cells[pos] != 0 {
                return true;
            }
        }

        false
    }

    /// Returns whether there are any notes for the cell at the given position.
    pub fn has_some_number(&self, pos: usize) -> bool {
        self.cells[pos] != 0
    }

    /// Removes all the notes that are invalidated by filling in the given
    /// number in the cell at the given coordinates.
    pub fn remove_all_notes_affected_by_set(&mut self, x: u8, y: u8, n: NonZeroU8) {
        let block_offset_x = get_block_offset(x);
        let block_offset_y = get_block_offset(y);

        for i in 0..9 {
            self.unset(x, y, NonZeroU8::new(i + 1).unwrap()); // Unset all notes in the cell.
            self.unset(i, y, n); // Unset the row.
            self.unset(x, i, n); // Unset the column.
            self.unset(block_offset_x + i % 3, block_offset_y + i / 3, n); // The block.
        }
    }

    /// Removes all the notes that are invalidated by the presence of a set of
    /// twins. This works for both regular twins and hidden twins.
    ///
    /// This does *not* remove the notes belonging to the twins themselves.
    ///
    /// Returns whether any notes were invalidated.
    pub fn remove_all_notes_affected_by_twins(&mut self, twins: Twins) -> bool {
        let mut eliminated_notes = false;
        let Twins {
            x1,
            y1,
            x2,
            y2,
            twin_notes,
        } = twins;

        if x1 == x2 {
            // Clear twins from column.
            for y in 0..9 {
                let notes_to_eliminate = if y == y1 || y == y2 {
                    !twin_notes
                } else {
                    twin_notes
                };
                if self.get(x1, y) & notes_to_eliminate != 0 {
                    self.cells[get_pos(x1, y)] &= !notes_to_eliminate;
                    eliminated_notes = true;
                }
            }
        } else if y1 == y2 {
            // Clear twins from row.
            for x in 0..9 {
                let notes_to_eliminate = if x == x1 || x == x2 {
                    !twin_notes
                } else {
                    twin_notes
                };
                if self.get(x, y1) & notes_to_eliminate != 0 {
                    self.cells[get_pos(x, y1)] &= !notes_to_eliminate;
                    eliminated_notes = true;
                }
            }
        }

        let block_offset_x = get_block_offset(x1);
        let block_offset_y = get_block_offset(y1);
        if block_offset_x == get_block_offset(x2) && block_offset_y == get_block_offset(y2) {
            // Clear twins from block.
            for i in 0..9 {
                let x = block_offset_x + i % 3;
                let y = block_offset_y + i / 3;
                let notes_to_eliminate = if (x == x1 && y == y1) || (x == x2 && y == y2) {
                    !twin_notes
                } else {
                    twin_notes
                };
                if self.get(x, y) & notes_to_eliminate != 0 {
                    self.cells[get_pos(x, y)] &= !notes_to_eliminate;
                    eliminated_notes = true;
                }
            }
        }

        eliminated_notes
    }

    /// Removes all the notes that are invalidated by the presene of a set of
    /// triplets. This works for both regular triplets and hidden triplets.
    ///
    /// This does *not* remove the notes belonging to the triplets themselves.
    ///
    /// Returns whether any notes were invalidated.
    pub fn remove_all_notes_affected_by_triplets(
        &mut self,
        Triplets {
            x1,
            y1,
            x2,
            y2,
            x3,
            y3,
            triplet_notes,
        }: Triplets,
    ) -> bool {
        let mut eliminated_notes = false;

        if x1 == x2 && x1 == x3 {
            // Clear triplets from column.
            for y in 0..9 {
                let notes_to_eliminate = if y == y1 || y == y2 || y == y3 {
                    !triplet_notes
                } else {
                    triplet_notes
                };
                if self.get(x1, y) & notes_to_eliminate != 0 {
                    self.cells[get_pos(x1, y)] &= !notes_to_eliminate;
                    eliminated_notes = true;
                }
            }
        } else if y1 == y2 && y1 == y3 {
            // Clear triplets from row.
            for x in 0..9 {
                let notes_to_eliminate = if x == x1 || x == x2 || x == x3 {
                    !triplet_notes
                } else {
                    triplet_notes
                };
                if self.get(x, y1) & notes_to_eliminate != 0 {
                    self.cells[get_pos(x, y1)] &= !notes_to_eliminate;
                    eliminated_notes = true;
                }
            }
        }

        let block_offset_x = get_block_offset(x1);
        let block_offset_y = get_block_offset(y1);
        if block_offset_x == get_block_offset(x2)
            && block_offset_y == get_block_offset(y2)
            && block_offset_x == get_block_offset(x3)
            && block_offset_y == get_block_offset(y3)
        {
            // Clear triplets from block.
            for i in 0..9 {
                let x = block_offset_x + i % 3;
                let y = block_offset_y + i / 3;
                let notes_to_eliminate =
                    if (x == x1 && y == y1) || (x == x2 && y == y2) || (x == x3 && y == y3) {
                        !triplet_notes
                    } else {
                        triplet_notes
                    };
                if self.get(x, y) & notes_to_eliminate != 0 {
                    self.cells[get_pos(x, y)] &= !notes_to_eliminate;
                    eliminated_notes = true;
                }
            }
        }

        eliminated_notes
    }

    /// Adds the given number to the notes for the cell at the given
    /// coordinates.
    ///
    /// Does nothing if the number was already in the notes.
    pub fn set(&mut self, x: u8, y: u8, n: NonZeroU8) {
        self.cells[get_pos(x, y)] |= 1 << n.get();
    }

    /// Toggles the given number in the notes for the cell at the given
    /// coordinates.
    pub fn toggle(&mut self, x: u8, y: u8, n: NonZeroU8) {
        if self.has(x, y, n) {
            self.unset(x, y, n)
        } else {
            self.set(x, y, n)
        }
    }

    /// Removes the given number from the notes for the cell at the given
    /// coordinates.
    ///
    /// Does nothing if the number wasn't in the notes.
    #[inline]
    pub fn unset(&mut self, x: u8, y: u8, n: NonZeroU8) {
        self.cells[get_pos(x, y)] &= !(1 << n.get());
    }
}

impl Default for Notes {
    fn default() -> Self {
        Self { cells: [0; 81] }
    }
}

fn get_num_notes(cell: u16) -> u8 {
    let mut num_notes = 0;
    let mut shifted_n = 2;
    while shifted_n <= 512 {
        if cell & shifted_n != 0 {
            num_notes += 1;
        }

        shifted_n <<= 1;
    }

    num_notes
}

fn get_twin_permutations(cell_notes: u16) -> Vec<u16> {
    let mut permutations = Vec::new();
    for i in 0..9 {
        for j in 0..9 {
            if i != j {
                let n = 2 << i | 2 << j;
                if cell_notes & n == n {
                    permutations.push(n);
                }
            }
        }
    }
    permutations
}

fn find_hidden_twin(twin_notes: u16, get_cell: impl Fn(u8) -> Option<u16>) -> Option<u8> {
    let mut other = None;
    for i in 0..9 {
        let Some(cell) = get_cell(i) else {
            continue;
        };

        if cell & twin_notes == twin_notes {
            if other.is_some() {
                return None; // The twin must only occur in one other place.
            } else {
                other = Some(i); // Found it, but make sure it's the only match.
            }
        } else if cell & twin_notes > 0 {
            return None; // One of the twin's numbers was found elsewhere.
        }
    }

    other
}

fn collect_notes_in_range(get_cell: impl Fn(u8) -> u16) -> u16 {
    let mut notes_in_range = 0;
    for i in 0..9 {
        notes_in_range |= get_cell(i);
    }

    notes_in_range
}

fn get_triplet_permutations(notes_in_range: u16) -> Vec<u16> {
    let mut permutations = Vec::new();
    for i in 0..9 {
        for j in 0..9 {
            if i != j {
                for k in 0..9 {
                    if k != i && k != j {
                        let n = 2 << i | 2 << j | 2 << k;
                        if notes_in_range & n == n {
                            permutations.push(n);
                        }
                    }
                }
            }
        }
    }
    permutations
}

fn find_hidden_triplets(
    triplet_notes: u16,
    get_cell: impl Fn(u8) -> Option<u16>,
) -> Option<(u8, u8)> {
    let mut other1 = None;
    let mut other2 = None;
    for i in 0..9 {
        let Some(cell) = get_cell(i) else {
            continue;
        };

        if cell & triplet_notes > 0 {
            if other1.is_some() {
                if other2.is_some() {
                    return None; // The triplets must occur in exactly two other places.
                } else {
                    other2 = Some(i); // Found it, but make sure it's the last match.
                }
            } else {
                other1 = Some(i); // Found it, but we need one more.
            }
        }
    }

    match (other1, other2) {
        (Some(i), Some(j)) => Some((i, j)),
        _ => None,
    }
}
