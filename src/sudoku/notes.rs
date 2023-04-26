use super::math::{get_block_offset, get_pos, get_x_and_y_from_pos};
use super::{Notes, Sudoku};
use std::num::NonZeroU8;

pub struct Twins {
    pub x1: u8,
    pub y1: u8,
    pub x2: u8,
    pub y2: u8,
}

pub struct Triplets {
    pub x1: u8,
    pub y1: u8,
    pub x2: u8,
    pub y2: u8,
    pub x3: u8,
    pub y3: u8,
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

    /// A set of twins are two cells that are within range of one another that
    /// both have the same numbers in their notes.
    ///
    /// This function checks if the cell at the given position is one of a set
    /// of twins and returns the coordinates for the twins if so.
    ///
    /// Assumes that all the notes are correctly filled in.
    pub fn find_twins(&self, pos: usize) -> Option<Twins> {
        let cell = self.cells[pos];
        if get_num_notes(cell) != 2 {
            return None;
        }

        let (x, y) = get_x_and_y_from_pos(pos);

        let block_offset_x = get_block_offset(x);
        let block_offset_y = get_block_offset(y);
        for i in 0..9 {
            // Check the row.
            if i != x && self.get(i, y) == cell {
                return Some(Twins {
                    x1: x,
                    y1: y,
                    x2: i,
                    y2: y,
                });
            }

            // Check the column.
            if i != y && self.get(x, i) == cell {
                return Some(Twins {
                    x1: x,
                    y1: y,
                    x2: x,
                    y2: i,
                });
            }

            // Check the block.
            let block_x = block_offset_x + i % 3;
            let block_y = block_offset_y + i / 3;
            if (block_x != x || block_y != y) && self.get(block_x, block_y) == cell {
                return Some(Twins {
                    x1: x,
                    y1: y,
                    x2: block_x,
                    y2: block_y,
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
        let cell = self.cells[pos];
        if get_num_notes(cell) != 3 {
            return None;
        }

        let (x, y) = get_x_and_y_from_pos(pos);

        let block_offset_x = get_block_offset(x);
        let block_offset_y = get_block_offset(y);
        for i in 0..9 {
            for j in 0..9 {
                if j == i {
                    continue;
                }

                // Check the row.
                if i != x && j != x && self.get(i, y) == cell && self.get(j, y) == cell {
                    return Some(Triplets {
                        x1: x,
                        y1: y,
                        x2: i,
                        y2: y,
                        x3: j,
                        y3: y,
                    });
                }

                // Check the column.
                if i != y && j != y && self.get(x, i) == cell && self.get(x, j) == cell {
                    return Some(Triplets {
                        x1: x,
                        y1: y,
                        x2: x,
                        y2: i,
                        x3: x,
                        y3: j,
                    });
                }

                // Check the block.
                let block_x1 = block_offset_x + i % 3;
                let block_y1 = block_offset_y + i / 3;
                let block_x2 = block_offset_x + j % 3;
                let block_y2 = block_offset_y + j / 3;
                if (block_x1 != x || block_y1 != y)
                    && (block_x2 != x || block_y2 != y)
                    && self.get(block_x1, block_y1) == cell
                    && self.get(block_x2, block_y2) == cell
                {
                    return Some(Triplets {
                        x1: x,
                        y1: y,
                        x2: block_x1,
                        y2: block_y1,
                        x3: block_x2,
                        y3: block_y2,
                    });
                }
            }
        }

        None
    }

    /// Returns the raw notes for the cell with the given coordinates.
    fn get(&self, x: u8, y: u8) -> u16 {
        self.cells[get_pos(x, y)]
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
            self.unset(x, y, NonZeroU8::new(i + 1).unwrap()); // Unset all numbers at the cell.
            self.unset(i, y, n); // Unset the row.
            self.unset(x, i, n); // Unset the column.
            self.unset(block_offset_x + i % 3, block_offset_y + i / 3, n); // The block.
        }
    }

    /// Removes all the notes that are invalidated by the presene of a set of
    /// twins.
    ///
    /// This does *not* remove notes from the cells that are part of the twins
    /// themselves.
    ///
    /// Returns whether any notes were invalidated.
    pub fn remove_all_notes_affected_by_twins(&mut self, Twins { x1, y1, x2, y2 }: Twins) -> bool {
        let mut eliminated_numbers = false;

        let cell = self.get(x1, y1);

        if x1 == x2 {
            // Clear twins from column.
            for y in 0..9 {
                if y != y1 && y != y2 && self.get(x1, y) & cell != 0 {
                    self.cells[get_pos(x1, y)] &= !cell;
                    eliminated_numbers = true;
                }
            }
        } else if y1 == y2 {
            // Clear twins from row.
            for x in 0..9 {
                if x != x1 && x != x2 && self.get(x, y1) & cell != 0 {
                    self.cells[get_pos(x, y1)] &= !cell;
                    eliminated_numbers = true;
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
                if (x != x1 || y != y1) && (x != x2 || y != y2) && self.get(x, y) & cell != 0 {
                    self.cells[get_pos(x, y)] &= !cell;
                    eliminated_numbers = true;
                }
            }
        }

        eliminated_numbers
    }

    /// Removes all the notes that are invalidated by the presene of a set of
    /// triplets.
    ///
    /// This does *not* remove notes from the cells that are part of the
    /// triplets themselves.
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
        }: Triplets,
    ) -> bool {
        let mut eliminated_numbers = false;

        let cell = self.get(x1, y1);

        if x1 == x2 && x1 == x3 {
            // Clear triplets from column.
            for y in 0..9 {
                if y != y1 && y != y2 && y != y3 && self.get(x1, y) & cell != 0 {
                    self.cells[get_pos(x1, y)] &= !cell;
                    eliminated_numbers = true;
                }
            }
        } else if y1 == y2 && y1 == y3 {
            // Clear triplets from row.
            for x in 0..9 {
                if x != x1 && x != x2 && x != x3 && self.get(x, y1) & cell != 0 {
                    self.cells[get_pos(x, y1)] &= !cell;
                    eliminated_numbers = true;
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
                if (x != x1 || y != y1)
                    && (x != x2 || y != y2)
                    && (x != x3 || y != y3)
                    && (self.cells[get_pos(x, y)] & cell) != 0
                {
                    self.cells[get_pos(x, y)] &= !cell;
                    eliminated_numbers = true;
                }
            }
        }

        eliminated_numbers
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
