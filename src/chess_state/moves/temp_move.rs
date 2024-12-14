use crate::chess_state::{board_bitmask::BoardBitmasks, chess_pieces::PieceEnum, coordinate_point::CoordinatePosition, moves::shared::{Move, MoveError}, moves::standard_move::StandardMove};

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
                takes,
            });

            output.push(next_move);

            move_copy &= !next_move_bitmask;
        }
    }
    Ok(output)
}