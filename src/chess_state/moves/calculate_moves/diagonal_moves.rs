use crate::chess_state::{
    board_bitmask::BoardBitmasks,
    chess_pieces::PieceEnum,
    coordinate_point::CoordinatePosition,
    coordinates::{XCoordinate, YCoordinate},
    moves::{
        shared::{Move, MoveError},
        standard_move::StandardMove,
    },
};

impl BoardBitmasks {
    fn calculate_diagonal_moves_up_right(
        &self,
        piece_type: PieceEnum,
    ) -> Result<Vec<Move>, MoveError> {
        use PieceEnum::*;
        // pieces from here will overflow the board
        const NOT_COLUMN_H_OR_ROW_8: u64 = !(XCoordinate::H as u64 | YCoordinate::Eight as u64);

        struct TempMove {
            moves: u64,
            captures: u64,
        }

        // bool to reflect if it is a white piece (true) or black piece (false) and filter invalid pieces
        let white = match piece_type {
            WhiteBishop | WhiteQueen => true,
            BlackBishop | BlackQueen => false,
            _ => return Err(MoveError::PieceCannotMoveDiagonally(piece_type)),
        };

        let own_pieces = match white {
            true => self.white_pieces.mask,
            false => self.black_pieces.mask,
        };

        let opponent_pieces = match white {
            true => self.black_pieces.mask,
            false => self.white_pieces.mask,
        };

        let starting_position = self.piece_enum_to_bitmask(piece_type);

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
            if previous_move.moves & previous_move.captures == 0 {
                // no previous moves, or all previous moves were captures (end of line)
                break;
            }
            let valid_moves = ((previous_move.moves & NOT_COLUMN_H_OR_ROW_8) << 9) & !own_pieces;
            let captures = valid_moves & opponent_pieces;
            packed_moves.push(TempMove {
                moves: valid_moves,
                captures,
            });
        }

        // now unpack the moves
        let output: Vec<Move> = Vec::new();
        for (index, packed_move) in packed_moves.iter().enumerate() {
            // guess to limit allocations
            let mut output: Vec<Move> = Vec::with_capacity(8);
            // take a copy of the move u64 to deconstruct
            let mut move_copy = packed_move.moves;
            while move_copy > 0 {
                let next_move_bitmask: u64 = 1 << move_copy.trailing_zeros();
                let next_move_coord = CoordinatePosition::from_bitmask(next_move_bitmask)?;
                let starting_pos_coord =
                    CoordinatePosition::from_bitmask(next_move_bitmask >> (9 * (index + 1)))?;
                let takes = match next_move_bitmask & packed_move.captures > 0 {
                    true => Some((
                        next_move_coord,
                        self.get_piece_type_for_capture(next_move_coord)?,
                    )),
                    false => None,
                };
                let next_move = Move::StandardMove(StandardMove {
                    start_position: starting_pos_coord,
                    end_position: next_move_coord,
                    piece: piece_type,
                    en_passant_target: None,
                    promotion: None,
                    takes,
                });

                output.push(next_move);

                move_copy &= !next_move_bitmask;
            }
        }
        Ok(output)
    }
}
