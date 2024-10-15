use std::{collections::btree_map::Values, error};

use thiserror::Error;

use crate::{
    bitmask::{black_pieces, generic::Bitmask, pieces::BitOpsForPieces, white_pieces}, BlackPawns, BlackPieces, WhitePawns, WhitePieces
};

use super::{board_bitmask::BoardBitmasks, chess_pieces::PieceEnum, coordinate_point::CoordinatePosition, coordinates::CoordinateError};

enum CastleType {
    ShortCastle,
    LongCastle
}

enum Move {
    StandardMove(StandardMove),
    Castle(CastleType),
}

struct StandardMove {
    start_position: CoordinatePosition,
    end_position: CoordinatePosition,
    piece: PieceEnum,
    en_passant_target: Option<CoordinatePosition>,
    promotion: Option<PieceEnum>,
    takes: Option<(CoordinatePosition, PieceEnum)>,
}

impl StandardMove {
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
    #[error("Capture piece not found at {0}")]
    CapturePieceNotFound(CoordinatePosition),
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

    fn calculate_white_pawn_moves_single_step(&self, occupied: u64) -> Result<Vec<Move>, MoveError> {
        let mut output: Vec<Move> = Vec::with_capacity(8);
        // no valid pawns on row 1
        // pawns on row 7 need to handle promotion moves
        // pawns on row 8 should already be promoted
        const ROWS_TWO_TO_SIX: u64 = 0x00_00_FF_FF_FF_FF_FF_00;
        let valid_pawns = self.white_pawns.mask & ROWS_TWO_TO_SIX;
        let mut valid_moves = (valid_pawns << 8) & !occupied;

        while valid_moves != 0 {
            let next_move = 1u64 << valid_moves.trailing_zeros(); // get next valid move
            let starting_position = next_move >> 8; // find the starting position

            output.push( // add to output
                Move::StandardMove(
                    create_simple_white_pawn_move(starting_position, next_move)?
                )
            );

            valid_moves &= !next_move; // remove that move
        }

        Ok(output) // return output
    }

fn calculate_white_pawn_moves_double_step(&self, occupied: u64) -> Result<Vec<Move>, MoveError> {
    let mut output: Vec<Move> = Vec::with_capacity(8);
    // only applies to pawns on row 2
    const ROW_TWO: u64 = 0xFF_00;
    let valid_pawns = self.white_pawns.mask & ROW_TWO;

    // need to ensure the pawns can step forwards once
    let valid_first_step = (valid_pawns << 8) & !occupied;

    // and again
    let mut valid_moves = (valid_first_step << 8) & !occupied;

    while valid_moves != 0 {
        let next_move = 1u64 << valid_moves.trailing_zeros(); // get next valid move
        let starting_position = next_move >> 16; // find the starting position two rows back

        output.push( // add to output
            Move::StandardMove(
                create_double_white_pawn_move(starting_position, next_move)?
            )
        );

        valid_moves &= !next_move; // remove that move
    }

    Ok(output)
}

    fn calculate_white_pawn_moves_capture_left(&self, occupied: u64) -> Result<Vec<Move>, MoveError> {}

    fn calculate_white_pawn_moves_capture_right(&self, occupied: u64) -> Result<Vec<Move>, MoveError> {}

    /// Calculates the en passant capture moves for white pawns.
    /// 
    /// En passant is a special capture move in chess that occurs when a pawn moves two squares forward
    /// from its starting position, and an opponent's pawn can capture it as if it had only moved one square.
    /// The en passant capture can only be made on the very next turn; otherwise, the opportunity is lost.
    /// 
    /// The en passant target is the square directly behind the opposing pawn that moved two squares.
    /// This function checks whether a white pawn can capture the black pawn via en passant and returns the possible move(s).
    /// 
    /// # Example:
    /// In the following example, a black pawn on D7 moves two squares forward to D5.
    /// A white pawn on C6 is now able to perform an en passant capture. The target square is D7 (x):
    ///
    /// ```
    ///      A   B   C   D   E   F   G   H
    ///  8 |   |   |   |   |   |   |   |   |
    ///  7 |   |   |   | x |   |   |   |   |
    ///  6 |   |   | P | p |   |   |   |   |
    ///  5 |   |   |   |   |   |   |   |   |
    /// ```
    ///
    /// After en passant is performed:
    ///
    /// ```
    ///      A   B   C   D   E   F   G   H
    ///  8 |   |   |   |   |   |   |   |   |
    ///  7 |   |   |   | P |   |   |   |   |
    ///  6 |   |   |   |   |   |   |   |   |
    ///  5 |   |   |   |   |   |   |   |   |
    /// ```
    /// 
    /// # Parameters:
    /// - `en_passant_target`: The coordinate of the en passant target square (the square where the white pawn will move if it performs en passant). This is `None` if en passant is not possible.
    /// 
    /// # Returns:
    /// - A `Vec<Move>` representing the valid en passant moves, or an empty vector if no en passant capture is possible.
    /// 
    /// # Errors:
    /// - Returns an error if the bitmask conversion for the starting or target positions fails.
    fn calculate_white_pawn_moves_en_passant(&self, en_passant_target: Option<CoordinatePosition>) -> Result<Vec<Move>, MoveError> {
        // only valid from row 6
        const ROW_SIX: u64 = 0x00_00_FF_00_00_00_00_00;
        if en_passant_target.is_none() {
            return Ok(Vec::new())
        }

        let mut output = Vec::with_capacity(2);

        let target_mask = en_passant_target.expect("Is not None").to_bitmask();
        // shift back and left and shift back and right to get the two valid spots
        // then & with ROW_SIX to ensure no overflow
        let valid_capture_positions = ((target_mask >> 7) | (target_mask >> 9)) & ROW_SIX;
        // check if there are any pawns occupying those positions
        let mut valid_pawns = self.white_pawns.mask & valid_capture_positions;
        while valid_pawns != 0 {
            let starting_position = 1u64 << valid_pawns.trailing_zeros();
            output.push(
                Move::StandardMove(StandardMove {
                    start_position: CoordinatePosition::from_bitmask(starting_position)?,
                    end_position: CoordinatePosition::from_bitmask(target_mask)?,
                    piece: PieceEnum::WhitePawn,
                    en_passant_target: None,
                    promotion: None,
                    takes: Some((
                        CoordinatePosition::from_bitmask(target_mask >> 8)?,
                        PieceEnum::BlackPawn
                    )),
                })
            );
            valid_pawns &= !starting_position; // remove pawn
        }

        Ok(output)        
    }

    fn calculate_white_pawn_promotions(&self, occupied: u64) -> Result<Vec<Move>, MoveError> {}

    fn get_piece_type_for_capture(&self, capture_position: CoordinatePosition) -> Result<PieceEnum, MoveError> {
        if (capture_position.to_bitmask() & self.all_pieces.to_u64()) == 0 {
            return Err(MoveError::CapturePieceNotFound(capture_position))
        } else {
            match capture_position {
                v if capture_position.to_bitmask() & self.white_pieces.to_u64() > 0 => {
                    // tis a white piece
                    match capture_position {
                        v if capture_position.to_bitmask() & self.white_pawns.to_u64() > 0 => {
                            Ok(PieceEnum::WhitePawn)
                        }
                        v if capture_position.to_bitmask() & self.white_knights.to_u64() > 0 => {
                            Ok(PieceEnum::WhiteKnight)
                        }
                        v if capture_position.to_bitmask() & self.white_bishops.to_u64() > 0 => {
                            Ok(PieceEnum::WhiteBishop)
                        }
                        v if capture_position.to_bitmask() & self.white_rooks.to_u64() > 0 => {
                            Ok(PieceEnum::WhiteRook)
                        }
                        v if capture_position.to_bitmask() & self.white_queens.to_u64() > 0 => {
                            Ok(PieceEnum::WhiteQueen)
                        }
                        v if capture_position.to_bitmask() & self.white_kings.to_u64() > 0 => {
                            Ok(PieceEnum::WhiteKing)
                        }
                        _ => Err(MoveError::CapturePieceNotFound(capture_position))
                    }
                }
                _ => {
                    // must be a black piece
                    match capture_position {
                        v if capture_position.to_bitmask() & self.black_pawns.to_u64() > 0 => {
                            Ok(PieceEnum::BlackPawn)
                        }
                        v if capture_position.to_bitmask() & self.black_knights.to_u64() > 0 => {
                            Ok(PieceEnum::BlackKnight)
                        }
                        v if capture_position.to_bitmask() & self.black_bishops.to_u64() > 0 => {
                            Ok(PieceEnum::BlackBishop)
                        }
                        v if capture_position.to_bitmask() & self.black_rooks.to_u64() > 0 => {
                            Ok(PieceEnum::BlackRook)
                        }
                        v if capture_position.to_bitmask() & self.black_queens.to_u64() > 0 => {
                            Ok(PieceEnum::BlackQueen)
                        }
                        v if capture_position.to_bitmask() & self.black_kings.to_u64() > 0 => {
                            Ok(PieceEnum::BlackKing)
                        }
                        _ => Err(MoveError::CapturePieceNotFound(capture_position))
                    }
                }
            }
        }
    }
}

fn create_simple_white_pawn_move(starting_position: u64, ending_position: u64) -> Result<StandardMove, MoveError> {
    let new_move = StandardMove::new(
        CoordinatePosition::from_bitmask(starting_position)?, 
        CoordinatePosition::from_bitmask(ending_position)?, 
        PieceEnum::WhitePawn, 
        None, 
        None, 
        None
    );
    Ok(new_move)
}

fn create_double_white_pawn_move(starting_position: u64, ending_position: u64) -> Result<StandardMove, MoveError> {
    let new_move = StandardMove::new(
        CoordinatePosition::from_bitmask(starting_position)?, 
        CoordinatePosition::from_bitmask(ending_position)?, 
        PieceEnum::WhitePawn, 
        // needs an en passant target
        Some(CoordinatePosition::from_bitmask(ending_position >> 8)?), 
        None, 
        None
    );
    Ok(new_move)
}