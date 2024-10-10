use std::error;

use thiserror::Error;

use crate::{
    bitmask::{black_pieces, generic::Bitmask, pieces::BitOpsForPieces, white_pieces}, BlackPawns, BlackPieces, WhitePawns, WhitePieces
};

use super::{board_bitmask::BoardBitmasks, chess_pieces::PieceEnum, coordinate_point::CoordinatePosition, coordinates::CoordinateError};

struct Move {
    start_position: CoordinatePosition,
    end_position: CoordinatePosition,
    piece: PieceEnum,
    en_passant_target: Option<CoordinatePosition>,
    promotion: Option<PieceEnum>,
    takes: Option<(CoordinatePosition, PieceEnum)>,
}

impl Move {
    pub(crate) fn new(
        start_position: CoordinatePosition,
        end_position: CoordinatePosition,
        piece: PieceEnum,
        en_passant_target: Option<CoordinatePosition>,
        promotion: Option<PieceEnum>,
        takes: Option<(CoordinatePosition, PieceEnum)>,
    ) -> Self {
        Self {
            start_position: start_position,
            end_position: end_position,
            promotion: promotion,
            takes: takes,
            piece: piece,
            en_passant_target: en_passant_target,
        }
    }

    pub(crate) fn get_uci_move(&self) -> String {
        let x = match self.takes {
            Some(_) => "x",
            None => "",
        };
        let promotion = match self.promotion {
            Some(piece) => piece.to_string(),
            None => "".to_string(),
        };
        format!(
            "{}{}{}{}",
            self.start_position, x, self.end_position, promotion
        )
    }
}

#[derive(Debug, Error)]
enum MoveError {
    #[error("Pawn found on rank one or eight (moved backwards or failed promotion")]
    PawnOnOneOrEight,
    #[error("Coordinate error: {0}")]
    CoordinateError(#[from] CoordinateError),
}

impl BoardBitmasks {
    pub(crate) fn calculate_white_pawn_moves(
        &self,
        en_passant: Option<CoordinatePosition>,
    ) -> Result<Vec<Move>, MoveError> {
        use crate::chess_state::coordinates::{
            XCoordinate::*,
            YCoordinate::*
        };
        // TODO: convert into smaller functions
        // prepare output with space for a sensible number of moves
        let mut output: Vec<Move> = Vec::with_capacity(16);
        // prepare some constants for later use
        let occupied: u64 = self.white_pieces.bitor_pieces(self.black_pieces).mask;
        let b_to_h: u64 = 0x7F7F7F7F7F7F7F7F;
        let a_to_g: u64 = 0xFEFEFEFEFEFEFEFE;
        // now on to the calculating
        let single_step = (self.white_pawns.mask << 8) & !occupied;
        let second_step =
            // only the white pawns on the second row, check they can move forwards once
            ((((self.white_pawns.mask & 0xFF00) << 8) & !occupied)
            // and then move them forward again, checking that they can
            << 8) & !occupied;
        let take_left = 
            // white pawns that can move left, bitshifted up a row and left
            ((self.white_pawns.mask & b_to_h) << 7)
            // and we need a black piece to take
            & self.black_pieces.mask;
        let take_right = 
            // white pawns that can move right, bitshifted up a row and right
            ((self.white_pawns.mask & a_to_g) << 9)
            // and we need a black piece to take
            & self.black_pieces.mask;
        let take_en_passant: Option<Vec<Move>> = match en_passant {
            Some(coordinate) => {
                if coordinate.y == Six { // should always be true
                    match coordinate.x {
                        A  => {
                            // check only B, so left shift
                            let valid_pawn_position = coordinate.to_bitmask() << 1;
                            if self.white_pawns.to_u64() & valid_pawn_position > 0 {
                                // there is a pawn on B6
                                Some(vec![
                                    Move::new(
                                        CoordinatePosition::from_str("b6")?,
                                        CoordinatePosition::from_str("a7")?,
                                        PieceEnum::WhitePawn,
                                        None,
                                        None,
                                        Some((coordinate, PieceEnum::BlackPawn))
                                    )
                                ])
                            } else {
                                None
                            }
                        }
                        H => {
                            // check only G, so right shift
                            let valid_pawn_position = coordinate.to_bitmask() >> 1;
                            if self.white_pawns.to_u64() & valid_pawn_position > 0 {
                                // there is a pawn on B6
                                Some(vec![
                                    Move::new(
                                        CoordinatePosition::from_str("g6")?,
                                        CoordinatePosition::from_str("h7")?,
                                        PieceEnum::WhitePawn,
                                        None,
                                        None,
                                        Some((coordinate, PieceEnum::BlackPawn))
                                    )
                                ])
                            } else {
                                None
                            }
                        }
                        _ => {
                            // now we need to check both left and right
                            let mut temp = Vec::new();
                            let left = coordinate.to_bitmask() >> 1;
                            if left & self.white_pawns.to_u64() > 0 {
                                temp.push(
                                    Move::new(
                                        CoordinatePosition::from_bitmask(left)?,
                                        // a row in front of the target pawn
                                        CoordinatePosition::from_bitmask(coordinate.to_bitmask() << 8)?,
                                        PieceEnum::WhitePawn,
                                        None,
                                        None,
                                        Some((coordinate, PieceEnum::BlackPawn))
                                    )
                                )
                            }
                            let right = coordinate.to_bitmask() << 1;
                            if right & self.white_pawns.to_u64() > 0 {
                                temp.push(
                                    Move::new(
                                        CoordinatePosition::from_bitmask(right)?,
                                        // a row in front of the target pawn
                                        CoordinatePosition::from_bitmask(coordinate.to_bitmask() << 8)?,
                                        PieceEnum::WhitePawn,
                                        None,
                                        None,
                                        Some((coordinate, PieceEnum::BlackPawn))
                                    )
                                )
                            }
                            // finally return
                            if temp.is_empty() {
                                None
                            } else {
                                Some(temp)
                            }
                        }
                    }
                } else {
                    panic!("Invalid en passant target given")
                }
            },
            None => None,
        };
        todo!()
    }
}