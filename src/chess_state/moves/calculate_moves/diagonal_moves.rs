use crate::chess_state::{
    board_bitmask::BoardBitmasks,
    chess_pieces::PieceEnum::{self, BlackBishop, BlackQueen, WhiteBishop, WhiteQueen},
    moves::{
        chess_move::{
            ChessDirection::{self, DownLeft, DownRight, UpLeft, UpRight},
            ChessShiftMove,
        },
        shared::{Move, MoveError},
        temp_move::{unpack_moves, TempMove},
    },
};

impl BoardBitmasks {
    /// Calculates all possible diagonal moves for a given piece type in a specified diagonal direction.
    ///
    /// This function determines the valid movement and capture positions for a white or black bishop
    /// or queen along a diagonal direction. It iterates through possible moves while ensuring that
    /// a piece does not move through its own pieces and only captures opponent pieces.
    ///
    /// # Arguments
    ///
    /// * `piece_type` - The type of the piece (must be a `WhiteBishop`, `BlackBishop`, `WhiteQueen`, or `BlackQueen`).
    /// * `diagonal_direction` - The direction in which to calculate diagonal moves (`UpRight`, `DownRight`, `DownLeft`, or `UpLeft`).
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing:
    /// - `Ok(Vec<Move>)` - A vector of valid moves for the piece.
    /// - `Err(MoveError)` - An error if the piece type is invalid or the direction is not diagonal.
    ///
    /// # Errors
    ///
    /// * `MoveError::InvalidPieceType` if the provided piece type is not a valid diagonal-moving piece.
    /// * `MoveError::InvalidDirection` if the given direction is not a valid diagonal direction.
    ///
    /// # Implementation Details
    ///
    /// * Determines whether the piece is white or black and retrieves the corresponding bitmask for its own and opponent pieces.
    /// * Iteratively shifts the piece's bitmask along the diagonal direction while ensuring it does not overlap with its own pieces.
    /// * Stops generating moves when encountering an occupied square (either capturing an opponent piece or reaching the board edge).
    /// * Uses `unpack_moves` to convert bitmask-based move data into a `Vec<Move>`.
    ///
    /// # Example Usage
    ///
    /// ```rust
    /// let moves = board.calculate_diagonal_moves_for_direction(PieceEnum::WhiteBishop, ChessDirection::UpRight)?;
    /// ```

    pub(crate) fn calculate_diagonal_moves_for_direction(
        &self,
        piece_type: PieceEnum,
        diagonal_direction: ChessDirection,
    ) -> Result<Vec<Move>, MoveError> {
        // bool to reflect if it is a white piece (true) or black piece (false) and filter invalid pieces
        let white = match piece_type {
            WhiteBishop | WhiteQueen => true,
            BlackBishop | BlackQueen => false,
            _ => {
                return Err(MoveError::InvalidPieceType(
                    "calculate_diagonal_moves_for_direction".into(),
                    format!("{:?}", [WhiteBishop, WhiteQueen, BlackBishop, BlackQueen]),
                    format!("{:?}", piece_type),
                ))
            }
        };

        // validate we have a valid diagonal direction and get the opposite direction for later undoing
        let reverse_direction = match diagonal_direction {
            UpRight => DownLeft,
            DownRight => UpLeft,
            DownLeft => UpRight,
            UpLeft => DownRight,
            _ => {
                return Err(MoveError::InvalidDirection(
                    "calculate_diagonal_moves_for_direction".into(),
                    format!("{:?}", [UpRight, DownRight, DownLeft, UpLeft]),
                    format!("{:?}", diagonal_direction),
                ))
            }
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
        let valid_moves = starting_position.shift_move(diagonal_direction) & !own_pieces;
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
            let valid_moves = (previous_move.moves.shift_move(diagonal_direction)) & !own_pieces;
            let captures = valid_moves & opponent_pieces;
            packed_moves.push(TempMove {
                moves: valid_moves,
                captures,
            });
        }

        unpack_moves(
            packed_moves,
            |bitmask, index| {
                (0..index).fold(bitmask, |current, _| current.shift_move(reverse_direction))
            },
            piece_type,
            &self,
        )
    }
}
