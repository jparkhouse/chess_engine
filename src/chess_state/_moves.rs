use thiserror::Error;

use crate::chess_state::coordinates::{XCoordinate, YCoordinate};

use super::{
    board_bitmask::BoardBitmasks, chess_pieces::PieceEnum, coordinate_point::CoordinatePosition,
    coordinates::CoordinateError,
};

enum CastleType {
    ShortCastle,
    LongCastle,
}

enum Move {
    StandardMove(StandardMove),
    Castle(CastleType),
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    #[error("Piece {0} cannot be moved diagonally")]
    PieceCannotMoveDiagonally(PieceEnum),
}

impl BoardBitmasks {
    /// Calculates all available white pawn moves and returns them as a Vec of Moves
    pub(crate) fn calculate_white_pawn_moves(
        &self,
        en_passant: Option<CoordinatePosition>,
    ) -> Result<Vec<Move>, MoveError> {
        let occupied = self.all_pieces.mask;

        let single_step_moves = self.calculate_white_pawn_moves_single_step(occupied)?;
        let double_step_moves = self.calculate_white_pawn_moves_double_step(occupied)?;
        let capture_left_moves = self.calculate_white_pawn_moves_capture_left()?;
        let capture_right_moves = self.calculate_white_pawn_moves_capture_right()?;
        let en_passant_moves = self.calculate_white_pawn_moves_en_passant(en_passant)?;
        let promotion_moves = self.calculate_white_pawn_promotions(occupied)?;

        // precalc size to avoid reallocation
        let move_count = single_step_moves.len()
            + double_step_moves.len()
            + capture_left_moves.len()
            + capture_right_moves.len()
            + en_passant_moves.len()
            + promotion_moves.len();
        let mut output: Vec<Move> = Vec::with_capacity(move_count);

        output.extend(single_step_moves);
        output.extend(double_step_moves);
        output.extend(capture_left_moves);
        output.extend(capture_right_moves);
        output.extend(en_passant_moves);
        output.extend(promotion_moves);

        Ok(output)
    }

    /// Calculates all available single step moves for white pawns and returns them as a Vec of Moves
    fn calculate_white_pawn_moves_single_step(
        &self,
        occupied: u64,
    ) -> Result<Vec<Move>, MoveError> {
        let mut output: Vec<Move> = Vec::with_capacity(8);
        // no valid pawns on row 1
        // pawns on row 7 need to handle promotion moves
        // pawns on row 8 should already be promoted
        const ROWS_TWO_TO_SIX: u64 =
            !(YCoordinate::One as u64 | YCoordinate::Seven as u64 | YCoordinate::Eight as u64);
        let valid_pawns = self.white_pawns.mask & ROWS_TWO_TO_SIX;
        let mut valid_moves = (valid_pawns << 8) & !occupied;

        while valid_moves != 0 {
            let next_move = 1u64 << valid_moves.trailing_zeros(); // get next valid move
            let starting_position = next_move >> 8; // find the starting position

            output.push(
                // add to output
                Move::StandardMove(create_simple_white_pawn_move(starting_position, next_move)?),
            );

            valid_moves &= !next_move; // remove that move
        }

        Ok(output) // return output
    }

    /// Calculates all available double-step moves for white pawns that have not moved yet, and returns a Vec of Moves
    fn calculate_white_pawn_moves_double_step(
        &self,
        occupied: u64,
    ) -> Result<Vec<Move>, MoveError> {
        let mut output: Vec<Move> = Vec::with_capacity(8);

        // only applies to pawns on row 2
        const ROW_TWO: u64 = YCoordinate::Two as u64;

        let valid_pawns = self.white_pawns.mask & ROW_TWO;

        // need to ensure the pawns can step forwards once
        let valid_first_step = (valid_pawns << 8) & !occupied;

        // and again
        let mut valid_moves = (valid_first_step << 8) & !occupied;

        while valid_moves != 0 {
            let next_move = 1u64 << valid_moves.trailing_zeros(); // get next valid move
            let starting_position = next_move >> 16; // find the starting position two rows back

            output.push(
                // add to output
                Move::StandardMove(create_double_white_pawn_move(starting_position, next_move)?),
            );

            valid_moves &= !next_move; // remove that move
        }

        Ok(output)
    }

    /// Calculates all available white pawn left-capture moves, and returns a Vec of Moves
    fn calculate_white_pawn_moves_capture_left(&self) -> Result<Vec<Move>, MoveError> {
        let mut output: Vec<Move> = Vec::with_capacity(8);

        // valid from rows 2-6 and only for pawns that can move left (ie not in column A)
        const VALID_SQUARES_NOT_IN_COLUMN_A: u64 = !(YCoordinate::One as u64
            | YCoordinate::Seven as u64
            | YCoordinate::Eight as u64
            | XCoordinate::A as u64);

        let valid_pawns = self.white_pawns.mask & VALID_SQUARES_NOT_IN_COLUMN_A;
        // valid moves move up and left one, and must capture a black piece
        let mut valid_captures = (valid_pawns.shift_move(ChessMove::UpLeft)) & self.black_pieces.mask;

        while valid_captures != 0 {
            let next_move = 1u64 << valid_captures.trailing_zeros(); // get next valid move
            let starting_position = next_move >> 7; // find the starting position one row back and to the right

            let coord_next_move = CoordinatePosition::from_bitmask(next_move)?;

            output.push(Move::StandardMove(StandardMove {
                start_position: CoordinatePosition::from_bitmask(starting_position)?,
                end_position: coord_next_move,
                piece: PieceEnum::WhitePawn,
                en_passant_target: None,
                promotion: None,
                takes: Some((
                    coord_next_move,
                    self.get_piece_type_for_capture(coord_next_move)?,
                )),
            }));

            valid_captures &= !next_move; // remove that move
        }

        Ok(output)
    }

    /// Calculates all available white pawn right-capture moves, and returns a Vec of Moves
    fn calculate_white_pawn_moves_capture_right(&self) -> Result<Vec<Move>, MoveError> {
        let mut output: Vec<Move> = Vec::with_capacity(8);

        // valid from rows 2-6 and only for pawns that can move right (ie not in column H)
        const VALID_SQUARES_NOT_IN_COLUMN_H: u64 = !(YCoordinate::One as u64
            | YCoordinate::Seven as u64
            | YCoordinate::Eight as u64
            | XCoordinate::H as u64);

        let valid_pawns = self.white_pawns.mask & VALID_SQUARES_NOT_IN_COLUMN_H;
        // valid moves move up and left one, and must capture a black piece
        let mut valid_captures = (valid_pawns << 9) & self.black_pieces.mask;

        while valid_captures != 0 {
            let next_move = 1u64 << valid_captures.trailing_zeros(); // get next valid move
            let starting_position = next_move >> 9; // find the starting position one row back and to the right

            let coord_next_move = CoordinatePosition::from_bitmask(next_move)?;

            output.push(Move::StandardMove(StandardMove {
                start_position: CoordinatePosition::from_bitmask(starting_position)?,
                end_position: coord_next_move,
                piece: PieceEnum::WhitePawn,
                en_passant_target: None,
                promotion: None,
                takes: Some((
                    coord_next_move,
                    self.get_piece_type_for_capture(coord_next_move)?,
                )),
            }));

            valid_captures &= !next_move; // remove that move
        }

        Ok(output)
    }

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
    fn calculate_white_pawn_moves_en_passant(
        &self,
        en_passant_target: Option<CoordinatePosition>,
    ) -> Result<Vec<Move>, MoveError> {
        // only valid from row 6
        const ROW_SIX: u64 = YCoordinate::Six as u64;
        if en_passant_target.is_none() {
            return Ok(Vec::new());
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
            output.push(Move::StandardMove(StandardMove {
                start_position: CoordinatePosition::from_bitmask(starting_position)?,
                end_position: CoordinatePosition::from_bitmask(target_mask)?,
                piece: PieceEnum::WhitePawn,
                en_passant_target: None,
                promotion: None,
                takes: Some((
                    CoordinatePosition::from_bitmask(target_mask >> 8)?,
                    PieceEnum::BlackPawn,
                )),
            }));
            valid_pawns &= !starting_position; // remove pawn
        }

        Ok(output)
    }

    /// Calculates available promotion moves, including step-forward, left-capture, and right-capture,
    /// and returns a move for each promotion option for each move as a Vec of Moves
    fn calculate_white_pawn_promotions(&self, occupied: u64) -> Result<Vec<Move>, MoveError> {
        let mut output: Vec<Move> = Vec::with_capacity(32);

        const ROW_SEVEN: u64 = YCoordinate::Seven as u64;
        const ROW_SEVEN_NOT_COLUMN_A: u64 = YCoordinate::Seven as u64 & !(XCoordinate::A as u64);
        const ROW_SEVEN_NOT_COLUMN_H: u64 = YCoordinate::Seven as u64 & !(XCoordinate::H as u64);

        let valid_pawns = self.white_pawns.mask & ROW_SEVEN;

        if valid_pawns == 0 {
            return Ok(output);
        }

        // there is at least one valid pawn
        let mut valid_move_forward = (valid_pawns << 8) & !occupied;
        while valid_move_forward != 0 {
            let next_move = 1u64 << valid_move_forward.trailing_zeros();
            let starting_position = next_move >> 8;

            let coord_next_move = CoordinatePosition::from_bitmask(next_move)?;
            let coord_starting_pos = CoordinatePosition::from_bitmask(starting_position)?;

            for piece in [
                PieceEnum::WhiteKnight,
                PieceEnum::WhiteBishop,
                PieceEnum::WhiteRook,
                PieceEnum::WhiteQueen,
            ] {
                output.push(Move::StandardMove(StandardMove {
                    start_position: coord_starting_pos,
                    end_position: coord_next_move,
                    piece: PieceEnum::WhitePawn,
                    en_passant_target: None,
                    promotion: Some(piece),
                    takes: None,
                }))
            }

            valid_move_forward &= !next_move;
        }

        let mut valid_capture_left =
            ((valid_pawns & ROW_SEVEN_NOT_COLUMN_A) << 7) & self.black_pieces.mask;
        while valid_capture_left != 0 {
            let next_move = 1u64 << valid_capture_left.trailing_zeros();
            let starting_position = next_move >> 7;

            let coord_next_move = CoordinatePosition::from_bitmask(next_move)?;
            let coord_starting_pos = CoordinatePosition::from_bitmask(starting_position)?;
            let captured_piece = self.get_piece_type_for_capture(coord_next_move)?;

            for piece in [
                PieceEnum::WhiteKnight,
                PieceEnum::WhiteBishop,
                PieceEnum::WhiteRook,
                PieceEnum::WhiteQueen,
            ] {
                output.push(Move::StandardMove(StandardMove {
                    start_position: coord_starting_pos,
                    end_position: coord_next_move,
                    piece: PieceEnum::WhitePawn,
                    en_passant_target: None,
                    promotion: Some(piece),
                    takes: Some((coord_next_move, captured_piece)),
                }))
            }

            valid_capture_left &= !next_move;
        }

        let mut valid_capture_right =
            ((valid_pawns & ROW_SEVEN_NOT_COLUMN_H) << 9) & self.black_pieces.mask;
        while valid_capture_right != 0 {
            let next_move = 1u64 << valid_capture_right.trailing_zeros();
            let starting_position = next_move >> 9;

            let coord_next_move = CoordinatePosition::from_bitmask(next_move)?;
            let coord_starting_pos = CoordinatePosition::from_bitmask(starting_position)?;
            let captured_piece = self.get_piece_type_for_capture(coord_next_move)?;

            for piece in [
                PieceEnum::WhiteKnight,
                PieceEnum::WhiteBishop,
                PieceEnum::WhiteRook,
                PieceEnum::WhiteQueen,
            ] {
                output.push(Move::StandardMove(StandardMove {
                    start_position: coord_starting_pos,
                    end_position: coord_next_move,
                    piece: PieceEnum::WhitePawn,
                    en_passant_target: None,
                    promotion: Some(piece),
                    takes: Some((coord_next_move, captured_piece)),
                }))
            }

            valid_capture_right &= !next_move;
        }

        Ok(output)
    }

    /// Calculates all possible diagonal moves in the up-right direction for bishops and queens.
    ///
    /// # Arguments
    ///
    /// * `piece_type` - The type of the piece (e.g., `WhiteBishop`, `BlackQueen`).
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `Move` instances if successful, or a `MoveError` if given an
    /// invalid piece that cannot move diagonally.
    fn calculate_diagonal_moves_up_right(
        &self,
        piece_type: PieceEnum,
    ) -> Result<Vec<Move>, MoveError> {
        use PieceEnum::*;
        // pieces from here will overflow the board
        const NOT_COLUMN_H_OR_ROW_8: u64 = !(XCoordinate::H as u64 | YCoordinate::Eight as u64);

        // bool to reflect if it is a white piece (true) or black piece (false) and filter invalid pieces
        let is_white = match piece_type {
            WhiteBishop | WhiteQueen => true,
            BlackBishop | BlackQueen => false,
            _ => return Err(MoveError::PieceCannotMoveDiagonally(piece_type)),
        };

        let own_pieces = match is_white {
            true => self.white_pieces.mask,
            false => self.black_pieces.mask,
        };

        let opponent_pieces = match is_white {
            true => self.black_pieces.mask,
            false => self.white_pieces.mask,
        };

        // check that white_bishops start from a sensible place, shift by 9 (row up, and one to right),
        // and then check they aren't on top of another white piece
        let valid_moves =
            ((self.piece_enum_to_bitmask(piece_type) & NOT_COLUMN_H_OR_ROW_8) << 9) & !own_pieces;
        let captures = valid_moves & opponent_pieces;

        let mut packed_moves = Vec::with_capacity(8);
        packed_moves.push(TempMove {
            moves: valid_moves,
            captures,
        });

        loop {
            let previous_move = packed_moves
                .last()
                .expect("Initialised with at least one value");
            if previous_move.moves == 0 || previous_move.captures - previous_move.moves == 0 {
                // no previous moves, or all previous moves were captures (end of line)
                break;
            }
            let valid_moves = ((previous_move.moves & NOT_COLUMN_H_OR_ROW_8) << 9) & !own_pieces;
            let captures = valid_moves & opponent_pieces;
            packed_moves.push(TempMove {
                moves: valid_moves,
                captures: captures,
            });
        }

        // now unpack the moves
        let output: Vec<Move> = unpack_moves(
            packed_moves,
            |bitmask, index| bitmask >> ((8 * index) + 1),
            piece_type,
            self,
        )?;
        Ok(output)
    }

    fn get_piece_type_for_capture(
        &self,
        capture_position: CoordinatePosition,
    ) -> Result<PieceEnum, MoveError> {
        if (capture_position.to_bitmask() & self.all_pieces.to_u64()) == 0 {
            return Err(MoveError::CapturePieceNotFound(capture_position));
        } else {
            match capture_position {
                _ if capture_position.to_bitmask() & self.white_pieces.to_u64() > 0 => {
                    // tis a white piece
                    match capture_position {
                        _ if capture_position.to_bitmask() & self.white_pawns.to_u64() > 0 => {
                            Ok(PieceEnum::WhitePawn)
                        }
                        _ if capture_position.to_bitmask() & self.white_knights.to_u64() > 0 => {
                            Ok(PieceEnum::WhiteKnight)
                        }
                        _ if capture_position.to_bitmask() & self.white_bishops.to_u64() > 0 => {
                            Ok(PieceEnum::WhiteBishop)
                        }
                        _ if capture_position.to_bitmask() & self.white_rooks.to_u64() > 0 => {
                            Ok(PieceEnum::WhiteRook)
                        }
                        _ if capture_position.to_bitmask() & self.white_queens.to_u64() > 0 => {
                            Ok(PieceEnum::WhiteQueen)
                        }
                        _ if capture_position.to_bitmask() & self.white_kings.to_u64() > 0 => {
                            Ok(PieceEnum::WhiteKing)
                        }
                        _ => Err(MoveError::CapturePieceNotFound(capture_position)),
                    }
                }
                _ => {
                    // must be a black piece
                    match capture_position {
                        _ if capture_position.to_bitmask() & self.black_pawns.to_u64() > 0 => {
                            Ok(PieceEnum::BlackPawn)
                        }
                        _ if capture_position.to_bitmask() & self.black_knights.to_u64() > 0 => {
                            Ok(PieceEnum::BlackKnight)
                        }
                        _ if capture_position.to_bitmask() & self.black_bishops.to_u64() > 0 => {
                            Ok(PieceEnum::BlackBishop)
                        }
                        _ if capture_position.to_bitmask() & self.black_rooks.to_u64() > 0 => {
                            Ok(PieceEnum::BlackRook)
                        }
                        _ if capture_position.to_bitmask() & self.black_queens.to_u64() > 0 => {
                            Ok(PieceEnum::BlackQueen)
                        }
                        _ if capture_position.to_bitmask() & self.black_kings.to_u64() > 0 => {
                            Ok(PieceEnum::BlackKing)
                        }
                        _ => Err(MoveError::CapturePieceNotFound(capture_position)),
                    }
                }
            }
        }
    }

    fn piece_enum_to_bitmask(&self, piece_type: PieceEnum) -> u64 {
        match piece_type {
            PieceEnum::WhitePawn => self.white_pawns.mask,
            PieceEnum::WhiteKnight => self.white_knights.mask,
            PieceEnum::WhiteBishop => self.white_bishops.mask,
            PieceEnum::WhiteRook => self.white_rooks.mask,
            PieceEnum::WhiteQueen => self.white_queens.mask,
            PieceEnum::WhiteKing => self.white_kings.mask,
            PieceEnum::BlackPawn => self.black_pawns.mask,
            PieceEnum::BlackKnight => self.black_knights.mask,
            PieceEnum::BlackBishop => self.black_bishops.mask,
            PieceEnum::BlackRook => self.black_rooks.mask,
            PieceEnum::BlackQueen => self.black_queens.mask,
            PieceEnum::BlackKing => self.black_kings.mask,
        }
    }
}

fn create_simple_white_pawn_move(
    starting_position: u64,
    ending_position: u64,
) -> Result<StandardMove, MoveError> {
    let new_move = StandardMove::new(
        CoordinatePosition::from_bitmask(starting_position)?,
        CoordinatePosition::from_bitmask(ending_position)?,
        PieceEnum::WhitePawn,
        None,
        None,
        None,
    );
    Ok(new_move)
}

fn create_double_white_pawn_move(
    starting_position: u64,
    ending_position: u64,
) -> Result<StandardMove, MoveError> {
    let new_move = StandardMove::new(
        CoordinatePosition::from_bitmask(starting_position)?,
        CoordinatePosition::from_bitmask(ending_position)?,
        PieceEnum::WhitePawn,
        // needs an en passant target
        Some(CoordinatePosition::from_bitmask(ending_position >> 8)?),
        None,
        None,
    );
    Ok(new_move)
}

struct TempMove {
    moves: u64,
    captures: u64,
}

/// Takes TempMoves which use bitmasks of multiple successive moves
fn unpack_moves<T: Fn(u64, usize) -> u64>(
    packed_moves: Vec<TempMove>,
    undo_moves: T,
    piece_type: PieceEnum,
    game_board: &BoardBitmasks,
) -> Result<Vec<Move>, MoveError> {
    let mut output: Vec<Move> = Vec::with_capacity(32);
    for (index, packed_move) in packed_moves.iter().enumerate() {
        // take a copy of the move u64 to deconstruct
        let mut move_copy = packed_move.moves;
        while move_copy > 0 {
            let next_move_bitmask: u64 = 1 << move_copy.trailing_zeros();
            let next_move_coord = CoordinatePosition::from_bitmask(next_move_bitmask)?;
            let starting_pos_coord =
                CoordinatePosition::from_bitmask(undo_moves(next_move_bitmask, index))?;
            let takes = match next_move_bitmask & packed_move.captures > 0 {
                true => Some((
                    next_move_coord,
                    game_board.get_piece_type_for_capture(next_move_coord)?,
                )),
                false => None,
            };
            let next_move = Move::StandardMove(StandardMove {
                start_position: starting_pos_coord,
                end_position: next_move_coord,
                piece: piece_type,
                en_passant_target: None,
                promotion: None,
                takes: takes,
            });

            output.push(next_move);

            move_copy &= !next_move_bitmask;
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    mod white_pawns {
        mod single_step_moves {
            use crate::chess_state::{
                coordinates::{XCoordinate, YCoordinate},
                _moves::{BoardBitmasks, Move},
            };

            #[test]
            fn all_pawns_can_step_forward_when_in_their_starting_position() {
                // arrange
                let mut game_board = BoardBitmasks::new();
                game_board.white_pawns.mask = 0x00_00_00_00_00_00_FF_00;
                game_board.white_pieces.mask = 0x00_00_00_00_00_00_FF_00;
                game_board.all_pieces.mask = 0x00_00_00_00_00_00_FF_00;

                // act
                let moves = game_board
                    .calculate_white_pawn_moves_single_step(0)
                    .expect("should produce 8 valid moves");

                let output_bitmask = moves.iter().fold(0, |bitmask: u64, m: &Move| match m {
                    Move::StandardMove(move_details) => {
                        bitmask | move_details.end_position.to_bitmask()
                    }
                    _ => panic!("No non-standard moves here!"),
                });

                // assert
                assert_eq!(moves.len(), 8); // there should be 8 valid moves
                assert_eq!(output_bitmask, 0x00_00_00_00_00_FF_00_00) // all pawns should move one step forwards
            }

            #[test]
            fn all_pawns_can_step_forward_when_in_valid_positions() {
                // arrange
                let mut game_board = BoardBitmasks::new();
                use XCoordinate::*;
                use YCoordinate::*;
                // invalid pawns on E8, C7, E1
                let invalid_pawns =
                    (E as u64 & Eight as u64) | (C as u64 & Seven as u64) | (E as u64 & One as u64);
                // valid pawns on A6, E6, G6, D5, B4, F4, H4, A2, C2, D2, F2, and H2
                let valid_pawns = (A as u64 & Six as u64)
                    | (E as u64 & Six as u64)
                    | (G as u64 & Six as u64)
                    | (D as u64 & Five as u64)
                    | (B as u64 & Four as u64)
                    | (F as u64 & Four as u64)
                    | (H as u64 & Four as u64)
                    | (A as u64 & Two as u64)
                    | (C as u64 & Two as u64)
                    | (D as u64 & Two as u64)
                    | (F as u64 & Two as u64)
                    | (H as u64 & Two as u64);
                game_board.white_pawns.mask = valid_pawns | invalid_pawns;
                let expected_output = valid_pawns << 8; // one step forwards

                // act
                let moves = game_board
                    .calculate_white_pawn_moves_single_step(0)
                    .expect("should produce 12 valid moves for 12 valid pawns");

                let output_bitmask = moves.iter().fold(0, |bitmask: u64, m: &Move| match m {
                    Move::StandardMove(move_details) => {
                        bitmask | move_details.end_position.to_bitmask()
                    }
                    _ => panic!("No non-standard moves here!"),
                });

                // assert
                assert_eq!(moves.len(), 12); // there should be 12 valid moves for 12 valid pawns
                assert_eq!(output_bitmask, expected_output) // all pawns should move one step forwards
            }

            #[test]
            fn pawns_in_invalid_positions_are_ignored_when_calculating_valid_moves() {
                // arrange
                let mut game_board = BoardBitmasks::new();
                game_board.white_pawns.mask = 0x00_00_00_00_00_00_00_FF;

                // act
                let moves = game_board
                    .calculate_white_pawn_moves_single_step(0)
                    .expect("should produce 0 valid moves");

                // assert
                assert_eq!(moves.len(), 0); // there should be no valid moves
            }

            #[test]
            fn blocked_pawn_cannot_step_forwards_when_calculating_valid_moves() {
                // arrange
                let mut game_board = BoardBitmasks::new();
                game_board.white_pawns.mask = 0x00_00_00_00_00_00_01_00;
                let occupied: u64 = 0x00_00_00_00_00_01_00_00; // blocks one pawn

                // act
                let moves = game_board
                    .calculate_white_pawn_moves_single_step(occupied)
                    .expect("should produce 0 valid moves");

                // assert
                assert_eq!(moves.len(), 0); // there should be no valid moves
            }

            #[test]
            fn other_pawns_can_step_forward_when_only_one_is_blocked() {
                // arrange
                let mut game_board = BoardBitmasks::new();
                game_board.white_pawns.mask = 0x00_00_00_00_00_00_FF_00;
                let occupied: u64 = 0x00_00_00_00_00_01_00_00; // blocks one pawn

                // act
                let moves = game_board
                    .calculate_white_pawn_moves_single_step(occupied)
                    .expect("should produce 7 valid moves");

                let output_bitmask = moves.iter().fold(0, |bitmask: u64, m: &Move| match m {
                    Move::StandardMove(move_details) => {
                        bitmask | move_details.end_position.to_bitmask()
                    }
                    _ => panic!("No non-standard moves here!"),
                });

                // assert
                assert_eq!(moves.len(), 7); // there should be 7 valid moves
                assert_eq!(output_bitmask, 0x00_00_00_00_00_FE_00_00) // from FF, only FE pawns should move one step forwards
                                                                      // since 01 pawn is blocked
            }
        }

        mod double_step_moves {
            use crate::chess_state::{
                coordinates::{XCoordinate, YCoordinate},
                _moves::{BoardBitmasks, Move},
            };

            #[test]
            fn all_pawns_can_step_forward_twice_when_in_their_starting_position() {
                // arrange
                let mut game_board = BoardBitmasks::new();
                game_board.white_pawns.mask = 0x00_00_00_00_00_00_FF_00;

                // act
                let moves = game_board
                    .calculate_white_pawn_moves_double_step(0)
                    .expect("should produce 8 valid moves");

                let output_bitmask = moves.iter().fold(0, |bitmask: u64, m: &Move| match m {
                    Move::StandardMove(move_details) => {
                        bitmask | move_details.end_position.to_bitmask()
                    }
                    _ => panic!("No non-standard moves here!"),
                });

                // assert
                assert_eq!(moves.len(), 8); // there should be 8 valid moves
                assert_eq!(output_bitmask, 0x00_00_00_00_FF_00_00_00) // all pawns should move two steps forwards
            }

            #[test]
            fn blocked_pawn_cannot_step_forwards_twice_when_calculating_valid_moves() {
                // arrange
                use XCoordinate::*;
                use YCoordinate::*;
                let mut game_board = BoardBitmasks::new();
                // start both A and B pawn in starting position
                game_board.white_pawns.mask = (A as u64 & Two as u64) | (B as u64 & Two as u64);
                // occupy one square in front of A pawn (A3) and two squares in front of B pawn (B4)
                let occupied: u64 = (A as u64 & Three as u64) | (B as u64 & Four as u64);

                // act
                let moves = game_board
                    .calculate_white_pawn_moves_double_step(occupied)
                    .expect("should produce 0 valid moves");

                // assert
                assert_eq!(moves.len(), 0); // there should be no valid moves
            }

            #[test]
            fn other_pawns_can_step_forward_twice_when_only_one_is_blocked() {
                // arrange
                let mut game_board = BoardBitmasks::new();
                game_board.white_pawns.mask = 0x00_00_00_00_00_00_FF_00;
                let occupied: u64 = 0x00_00_00_00_00_01_00_00; // blocks one pawn

                // act
                let moves = game_board
                    .calculate_white_pawn_moves_double_step(occupied)
                    .expect("should produce 7 valid moves");

                let output_bitmask = moves.iter().fold(0, |bitmask: u64, m: &Move| match m {
                    Move::StandardMove(move_details) => {
                        bitmask | move_details.end_position.to_bitmask()
                    }
                    _ => panic!("No non-standard moves here!"),
                });

                // assert
                assert_eq!(moves.len(), 7); // there should be 7 valid moves
                assert_eq!(output_bitmask, 0x00_00_00_00_FE_00_00_00) // from FF, only FE pawns should move two step forwards
                                                                      // since 01 pawn is blocked
            }

            #[test]
            fn pawns_in_invalid_positions_are_ignored_when_calculating_valid_moves() {
                // arrange
                let mut game_board = BoardBitmasks::new();
                // only pawns on row 2 are valid
                game_board.white_pawns.mask = 0xFF_FF_FF_FF_FF_FF_00_FF;

                // act
                let moves = game_board
                    .calculate_white_pawn_moves_double_step(0)
                    .expect("should produce 0 valid moves");

                // assert
                assert_eq!(moves.len(), 0); // there should be no valid moves
            }

            #[test]
            fn all_pawns_can_step_forward_twice_when_in_valid_positions() {
                // arrange
                let mut game_board = BoardBitmasks::new();
                use XCoordinate::*;
                use YCoordinate::*;
                // invalid pawns on E8, C7, A6, E6, G6, D5, B4, F4, H4, E1
                let invalid_pawns = (E as u64 & Eight as u64)
                    | (C as u64 & Seven as u64)
                    | (E as u64 & One as u64)
                    | (A as u64 & Six as u64)
                    | (E as u64 & Six as u64)
                    | (G as u64 & Six as u64)
                    | (D as u64 & Five as u64)
                    | (B as u64 & Four as u64)
                    | (F as u64 & Four as u64)
                    | (H as u64 & Four as u64);
                // valid pawns on A2, C2, D2, F2, and H2
                let valid_pawns = (A as u64 & Two as u64)
                    | (C as u64 & Two as u64)
                    | (D as u64 & Two as u64)
                    | (F as u64 & Two as u64)
                    | (H as u64 & Two as u64);
                game_board.white_pawns.mask = valid_pawns | invalid_pawns;
                let expected_output = valid_pawns << 16; // two step forwards

                // act
                let moves = game_board
                    .calculate_white_pawn_moves_double_step(0)
                    .expect("should produce 5 valid moves for 5 valid pawns");

                let output_bitmask = moves.iter().fold(0, |bitmask: u64, m: &Move| match m {
                    Move::StandardMove(move_details) => {
                        bitmask | move_details.end_position.to_bitmask()
                    }
                    _ => panic!("No non-standard moves here!"),
                });

                // assert
                assert_eq!(moves.len(), 5); // there should be 5 valid moves for 5 valid pawns
                assert_eq!(output_bitmask, expected_output) // all pawns should move two step forwards
            }
        }

        mod capture_left_moves {
            use crate::chess_state::{
                coordinates::{XCoordinate, YCoordinate},
                _moves::{BoardBitmasks, CoordinatePosition, Move, PieceEnum, StandardMove},
            };

            #[test]
            fn no_captures_when_there_are_no_capture_targets() {
                // arrange
                let mut game_board = BoardBitmasks::new();
                // white pawn starting position
                game_board.white_pawns.mask = 0x00_00_00_00_00_00_FF_00;
                // every other mask is 0

                // act
                let available_left_captures = game_board
                    .calculate_white_pawn_moves_capture_left()
                    .expect("should generate 0 valid moves");

                // assert
                assert_eq!(available_left_captures.len(), 0)
            }

            #[test]
            fn identifies_valid_capture_when_caputurable_piece_to_the_left() {
                // arrange
                let mut game_board = BoardBitmasks::new();
                let white_pawn_position = XCoordinate::E as u64 & YCoordinate::Two as u64;
                let black_rook_position = XCoordinate::D as u64 & YCoordinate::Three as u64;
                // update gameboard to respect this
                game_board.white_pawns.mask = white_pawn_position;
                game_board.white_pieces.mask = white_pawn_position;
                game_board.black_rooks.mask = black_rook_position;
                game_board.black_pieces.mask = black_rook_position;
                game_board.all_pieces.mask = white_pawn_position | black_rook_position;

                let expected_capture = StandardMove {
                    start_position: CoordinatePosition::from_str("e2").expect("valid position"),
                    end_position: CoordinatePosition::from_str("d3").expect("valid position"),
                    piece: PieceEnum::WhitePawn,
                    en_passant_target: None,
                    promotion: None,
                    takes: Some((
                        CoordinatePosition::from_str("d3").expect("valid position"),
                        PieceEnum::BlackRook,
                    )),
                };

                // act
                let capture = match game_board
                    .calculate_white_pawn_moves_capture_left()
                    .expect("should generate one valid move")
                    .first()
                    .expect("should contain one valid move")
                {
                    Move::StandardMove(capture) => capture.clone(),
                    _ => panic!("only standard moves here"),
                };

                // assert
                assert_eq!(capture, expected_capture)
            }
        }
    }
}
