use crate::{
    bitmask::generic::Bitmask, BlackBishops, BlackKings, BlackKnights, BlackPawns, BlackPieces,
    BlackQueens, BlackRooks, Pieces, WhiteBishops, WhiteKings, WhiteKnights,
    WhitePawns, WhitePieces, WhiteQueens, WhiteRooks,
};

use super::board_hash_map::BoardHashMap;

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

    pub(crate) fn from_board_hash_map(map: &BoardHashMap) -> Self {
        use crate::PieceEnum::*;

        let mut output = Self::new();
        map.to_iter().for_each(|(coord, piece)| match piece {
            WhitePawn => {
                let bitmask: Bitmask<WhitePawns> = Bitmask::from_u64(coord.to_bitmask());
                output.white_pawns |= bitmask.clone();
                output.white_pieces |= bitmask.into();
                output.all_pieces |= bitmask.into();
            }
            WhiteKnight => {
                let bitmask: Bitmask<WhiteKnights> = Bitmask::from_u64(coord.to_bitmask());
                output.white_knights |= bitmask.clone();
                output.white_pieces |= bitmask.into();
                output.all_pieces |= bitmask.into();
            }
            WhiteBishop => {
                let bitmask: Bitmask<WhiteBishops> = Bitmask::from_u64(coord.to_bitmask());
                output.white_bishops |= bitmask.clone();
                output.white_pieces |= bitmask.into();
                output.all_pieces |= bitmask.into();
            }
            WhiteRook => {
                let bitmask: Bitmask<WhiteRooks> = Bitmask::from_u64(coord.to_bitmask());
                output.white_rooks |= bitmask.clone();
                output.white_pieces |= bitmask.into();
                output.all_pieces |= bitmask.into();
            }
            WhiteQueen => {
                let bitmask: Bitmask<WhiteQueens> = Bitmask::from_u64(coord.to_bitmask());
                output.white_queens |= bitmask.clone();
                output.white_pieces |= bitmask.into();
                output.all_pieces |= bitmask.into();
            }
            WhiteKing => {
                let bitmask: Bitmask<WhiteKings> = Bitmask::from_u64(coord.to_bitmask());
                output.white_kings |= bitmask.clone();
                output.white_pieces |= bitmask.into();
                output.all_pieces |= bitmask.into();
            }
            BlackPawn => {
                let bitmask: Bitmask<BlackPawns> = Bitmask::from_u64(coord.to_bitmask());
                output.black_pawns |= bitmask.clone();
                output.black_pieces |= bitmask.into();
                output.all_pieces |= bitmask.into();
            }
            BlackKnight => {
                let bitmask: Bitmask<BlackKnights> = Bitmask::from_u64(coord.to_bitmask());
                output.black_knights |= bitmask.clone();
                output.black_pieces |= bitmask.into();
                output.all_pieces |= bitmask.into();
            }
            BlackBishop => {
                let bitmask: Bitmask<BlackBishops> = Bitmask::from_u64(coord.to_bitmask());
                output.black_bishops |= bitmask.clone();
                output.black_pieces |= bitmask.into();
                output.all_pieces |= bitmask.into();
            }
            BlackRook => {
                let bitmask: Bitmask<BlackRooks> = Bitmask::from_u64(coord.to_bitmask());
                output.black_rooks |= bitmask.clone();
                output.black_pieces |= bitmask.into();
                output.all_pieces |= bitmask.into();
            }
            BlackQueen => {
                let bitmask: Bitmask<BlackQueens> = Bitmask::from_u64(coord.to_bitmask());
                output.black_queens |= bitmask.clone();
                output.black_pieces |= bitmask.into();
                output.all_pieces |= bitmask.into();
            }
            BlackKing => {
                let bitmask: Bitmask<BlackKings> = Bitmask::from_u64(coord.to_bitmask());
                output.black_kings |= bitmask.clone();
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
