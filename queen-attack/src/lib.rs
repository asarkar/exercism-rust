use std::cmp;

#[derive(Debug)]
pub struct ChessPosition {
    rank: u8,
    file: u8,
}

#[derive(Debug)]
pub struct Queen {
    pos: ChessPosition,
}

impl ChessPosition {
    const BOARD_SIZE: i32 = 8;

    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if (0..ChessPosition::BOARD_SIZE).contains(&rank)
            && (0..ChessPosition::BOARD_SIZE).contains(&file)
        {
            Some(Self {
                rank: rank as u8,
                file: file as u8,
            })
        } else {
            None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { pos: position }
    }

    /*
     * Checking for row and column attacks are easy. Diagonal attacks are little tricky.
     * To know if the current (row, column) falls in a diagonal under attack we note
     * that the coordinates of a cell under attack diagonally from a queen has a symmetry:
     * the difference between the current row and the row of the queen is equal to the
     * difference between the current column and the column of the queen.
     *
     * For example, on a 4x4 board, a queen placed at (0,1) will attack all of (1,0), (1,2),
     * and (2,3).
     * A queen placed at (2,0) will attack (1,1), (0,2), and (3,1).
     */
    pub fn can_attack(&self, other: &Queen) -> bool {
        self.pos.rank == other.pos.rank
            || self.pos.file == other.pos.file
            || (cmp::max(self.pos.rank, other.pos.rank) - cmp::min(self.pos.rank, other.pos.rank))
                == (cmp::max(self.pos.file, other.pos.file)
                    - cmp::min(self.pos.file, other.pos.file))
    }
}
