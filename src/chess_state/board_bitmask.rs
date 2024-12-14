use crate::{
    bitmask::generic::Bitmask, BlackBishops, BlackKings, BlackKnights, BlackPawns, BlackPieces,
    BlackQueens, BlackRooks, Pieces, WhiteBishops, WhiteKings, WhiteKnights,
    WhitePawns, WhitePieces, WhiteQueens, WhiteRooks,
};

use super::{board_hash_map::BoardHashMap, coordinates::{YCoordinate, XCoordinate}};

pub(crate) struct BoardBitmasks {
    pub all_pieces: Bitmask<Pieces>,
    pub white_pieces: Bitmask<WhitePieces>,
    pub white_pawns: Bitmask<WhitePawns>,
    pub white_knights: Bitmask<WhiteKnights>,
    pub white_bishops: Bitmask<WhiteBishops>,
    pub white_rooks: Bitmask<WhiteRooks>,
    pub white_queens: Bitmask<WhiteQueens>,
    pub white_kings: Bitmask<WhiteKings>,
    pub black_pieces: Bitmask<BlackPieces>,
    pub black_pawns: Bitmask<BlackPawns>,
    pub black_knights: Bitmask<BlackKnights>,
    pub black_bishops: Bitmask<BlackBishops>,
    pub black_rooks: Bitmask<BlackRooks>,
    pub black_queens: Bitmask<BlackQueens>,
    pub black_kings: Bitmask<BlackKings>,
}

impl BoardBitmasks {
    /// Initialises a new, empty, chess board and the relevant bitmasks
    pub(crate) fn new() -> Self {
        Self {
            all_pieces: 0.into(),
            white_pieces: 0.into(),
            white_pawns: 0.into(),
            white_knights: 0.into(),
            white_bishops: 0.into(),
            white_rooks: 0.into(),
            white_queens: 0.into(),
            white_kings: 0.into(),
            black_pieces: 0.into(),
            black_pawns: 0.into(),
            black_knights: 0.into(),
            black_bishops: 0.into(),
            black_rooks: 0.into(),
            black_queens: 0.into(),
            black_kings: 0.into(),
        }
    }

    /// Initialises a chess board in the starting position
    pub(crate) fn default() -> Self {
        use XCoordinate::*;
        use YCoordinate::*;
        Self {
            all_pieces:     (One as u64 | Two as u64 | Seven as u64 | Eight as u64).into(),
            white_pieces:   (One as u64 | Two as u64).into(),
            white_pawns:    (Two as u64).into(),
            white_knights:  ((B as u64 & One as u64) | (G as u64 & One as u64)).into(),
            white_bishops:  ((C as u64 & One as u64) | (F as u64 & One as u64)).into(),
            white_rooks:    ((A as u64 & One as u64) | (H as u64 & One as u64)).into(),
            white_queens:   (D as u64 & One as u64).into(),
            white_kings:    (E as u64 & One as u64).into(),
            black_pieces:   (Seven as u64 | Eight as u64).into(),
            black_pawns:    (Seven as u64).into(),
            black_knights:  ((B as u64 & Eight as u64) | (G as u64 & Eight as u64)).into(),
            black_bishops:  ((C as u64 & Eight as u64) | (F as u64 & Eight as u64)).into(),
            black_rooks:    ((A as u64 & Eight as u64) | (H as u64 & Eight as u64)).into(),
            black_queens:   (D as u64 & Eight as u64).into(),
            black_kings:    (E as u64 & Eight as u64).into(),
        }
    }

    /// Creates a BoardBitmasks object from a BoardHashMap, going from location-to-piece to piece-to-location
    pub(crate) fn from_board_hash_map(map: &BoardHashMap) -> Self {
        use crate::PieceEnum::*;

        let mut output = Self::new();
        map.to_iter().for_each(|(coord, piece)| match piece {
            WhitePawn => {
                let bitmask: Bitmask<WhitePawns> = Bitmask::from_u64(coord.to_bitmask());
                output.white_pawns |= bitmask;
                output.white_pieces |= bitmask.into();
                output.all_pieces |= bitmask.into();
            }
            WhiteKnight => {
                let bitmask: Bitmask<WhiteKnights> = Bitmask::from_u64(coord.to_bitmask());
                output.white_knights |= bitmask;
                output.white_pieces |= bitmask.into();
                output.all_pieces |= bitmask.into();
            }
            WhiteBishop => {
                let bitmask: Bitmask<WhiteBishops> = Bitmask::from_u64(coord.to_bitmask());
                output.white_bishops |= bitmask;
                output.white_pieces |= bitmask.into();
                output.all_pieces |= bitmask.into();
            }
            WhiteRook => {
                let bitmask: Bitmask<WhiteRooks> = Bitmask::from_u64(coord.to_bitmask());
                output.white_rooks |= bitmask;
                output.white_pieces |= bitmask.into();
                output.all_pieces |= bitmask.into();
            }
            WhiteQueen => {
                let bitmask: Bitmask<WhiteQueens> = Bitmask::from_u64(coord.to_bitmask());
                output.white_queens |= bitmask;
                output.white_pieces |= bitmask.into();
                output.all_pieces |= bitmask.into();
            }
            WhiteKing => {
                let bitmask: Bitmask<WhiteKings> = Bitmask::from_u64(coord.to_bitmask());
                output.white_kings |= bitmask;
                output.white_pieces |= bitmask.into();
                output.all_pieces |= bitmask.into();
            }
            BlackPawn => {
                let bitmask: Bitmask<BlackPawns> = Bitmask::from_u64(coord.to_bitmask());
                output.black_pawns |= bitmask;
                output.black_pieces |= bitmask.into();
                output.all_pieces |= bitmask.into();
            }
            BlackKnight => {
                let bitmask: Bitmask<BlackKnights> = Bitmask::from_u64(coord.to_bitmask());
                output.black_knights |= bitmask;
                output.black_pieces |= bitmask.into();
                output.all_pieces |= bitmask.into();
            }
            BlackBishop => {
                let bitmask: Bitmask<BlackBishops> = Bitmask::from_u64(coord.to_bitmask());
                output.black_bishops |= bitmask;
                output.black_pieces |= bitmask.into();
                output.all_pieces |= bitmask.into();
            }
            BlackRook => {
                let bitmask: Bitmask<BlackRooks> = Bitmask::from_u64(coord.to_bitmask());
                output.black_rooks |= bitmask;
                output.black_pieces |= bitmask.into();
                output.all_pieces |= bitmask.into();
            }
            BlackQueen => {
                let bitmask: Bitmask<BlackQueens> = Bitmask::from_u64(coord.to_bitmask());
                output.black_queens |= bitmask;
                output.black_pieces |= bitmask.into();
                output.all_pieces |= bitmask.into();
            }
            BlackKing => {
                let bitmask: Bitmask<BlackKings> = Bitmask::from_u64(coord.to_bitmask());
                output.black_kings |= bitmask;
                output.black_pieces |= bitmask.into();
                output.all_pieces |= bitmask.into();
            }
        });
        output
    }
}

#[cfg(test)]
mod tests {
    mod unit_tests {
        mod new {
            use crate::chess_state::board_bitmask::BoardBitmasks;

            #[test]
            fn initialises_empty_bitmask_board_when_using_new() {
                // arrange + act
                let board = BoardBitmasks::new();
                // assert
                assert_eq!(board.all_pieces.to_u64(), 0);
                assert_eq!(board.white_pieces.to_u64(), 0);
                assert_eq!(board.black_pieces.to_u64(), 0);
                assert_eq!(board.white_pawns.to_u64(), 0);
                assert_eq!(board.white_knights.to_u64(), 0);
                assert_eq!(board.white_bishops.to_u64(), 0);
                assert_eq!(board.white_rooks.to_u64(), 0);
                assert_eq!(board.white_queens.to_u64(), 0);
                assert_eq!(board.white_kings.to_u64(), 0);
                assert_eq!(board.black_pawns.to_u64(), 0);
                assert_eq!(board.black_knights.to_u64(), 0);
                assert_eq!(board.black_bishops.to_u64(), 0);
                assert_eq!(board.black_rooks.to_u64(), 0);
                assert_eq!(board.black_queens.to_u64(), 0);
                assert_eq!(board.black_kings.to_u64(), 0);
            }
        }
    }

    mod from_board_hash_map {
        // needs an almost integration test approach
    }
}
