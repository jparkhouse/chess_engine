// figure out how to return a bitmask of pinned pieces

use crate::chess_state::{
    board_bitmask::BoardBitmasks,
    moves::{
        attack_maps::{BishopAttackMaps, RookAttackMaps},
        chess_move::{ChessDirection, ChessShiftMove},
        shared::MoveError,
    },
};

impl BoardBitmasks {
    /// Checks for any pieces that are geometrically pinned to the king from any direction. Returns a bitmask (`u64`) of those pinned pieces.
    /// Bear in mind that these pinned pieces may still have valid moves like moving along the line of the pin or capturing the pinning piece.
    /// Takes only `white: bool`, which informs if we are checking for pins against the white king (`true`) or the black king (`false`).
    pub(crate) fn get_pieces_pinned_to_king(&self, white: bool) -> u64 {
        self.get_pieces_cardinally_pinned_to_king(white)
            | self.get_pieces_diagonally_pinned_to_king(white)
    }

    /// Checks for any pieces that are 'cardinally' pinned to the king (i.e. above, below, or to the side).
    /// Returns a bitmask (`u64`) of those pinned pieces.
    ///
    /// Takes only `white: bool`, which informs if we are checking for pins against the white king (`true`) or the black king (`false`).
    pub(crate) fn get_pieces_cardinally_pinned_to_king(&self, white: bool) -> u64 {
        // initialise our empty bitmask
        let mut output: u64 = 0;

        // figure out which side we are looking for
        let king_bitmask = match white {
            true => self.white_kings.mask,
            false => self.black_kings.mask,
        };

        // figure out our attacking pieces
        let (off_rook_bitmask, off_queen_bitmask) = match white {
            true => (self.black_rooks.mask, self.black_queens.mask),
            false => (self.white_rooks.mask, self.white_queens.mask),
        };

        // figure out our defending pieces
        let def_piece_bitmask = match white {
            true => self.white_pieces.mask,
            false => self.black_pieces.mask,
        };

        // now we can work from the king outwards and see if we have any pieces in his rays
        let king_cardinal_attack_squares = king_bitmask.calculate_unconstrained_rook_attack_maps();
        let cardinal_attackers = off_rook_bitmask | off_queen_bitmask;
        if king_cardinal_attack_squares & cardinal_attackers != 0 {
            // there is at least one queen or rook that could generate a pin in a cardinal direction
            for cardinal_direction in [
                ChessDirection::Up,
                ChessDirection::Right,
                ChessDirection::Down,
                ChessDirection::Left,
            ] {
                let pinned_piece_position = check_for_pin(
                    king_bitmask,
                    cardinal_direction,
                    cardinal_attackers,
                    def_piece_bitmask,
                );
                match pinned_piece_position {
                    Ok(piece) => output |= piece,
                    // should never error since there is no chance of passing in bad direction
                    Err(_) => {}
                }
            }
        }

        output
    }

    /// Checks for any pieces that are 'diagonally' pinned to the king. Returns a bitmask (`u64`) of those pinned pieces.
    /// Takes only `white: bool`, which informs if we are checking for pins against the white king (`true`) or the black king (`false`).
    pub(crate) fn get_pieces_diagonally_pinned_to_king(&self, white: bool) -> u64 {
        // initialise our empty bitmask
        let mut output: u64 = 0;

        // figure out which side we are looking for
        let king_bitmask = match white {
            true => self.white_kings.mask,
            false => self.black_kings.mask,
        };

        // figure out our attacking pieces
        let (off_bishop_bitmask, off_queen_bitmask) = match white {
            true => (self.black_bishops.mask, self.black_queens.mask),
            false => (self.white_bishops.mask, self.white_queens.mask),
        };

        // figure out our defending pieces
        let def_piece_bitmask = match white {
            true => self.white_pieces.mask,
            false => self.black_pieces.mask,
        };

        // now we can work from the king outwards and identify any pins in his rays
        let king_diagonal_attack_squares =
            king_bitmask.calculate_unconstrained_bishop_attack_maps();
        let diagonal_attackers = off_bishop_bitmask | off_queen_bitmask;
        if king_diagonal_attack_squares & diagonal_attackers != 0 {
            // there is at least one queen or bishop that could generate a pin in a cardinal direction
            for diagonal_direction in [
                ChessDirection::UpRight,
                ChessDirection::DownRight,
                ChessDirection::DownLeft,
                ChessDirection::UpLeft,
            ] {
                let pinned_piece_position = check_for_pin(
                    king_bitmask,
                    diagonal_direction,
                    diagonal_attackers,
                    def_piece_bitmask,
                );
                match pinned_piece_position {
                    Ok(piece) => output |= piece,
                    // should never error since there is no chance of passing in bad direction
                    Err(_) => {}
                }
            }
        }

        output
    }
}

/// Casts a ray from the `king_position`, and checks that as you progress outwards from the king in `direction`,
/// you encounter exactly one of the `defending_pieces` and then exactly one of the `attacking_pieces`.
/// Only valid for cardinal or diagonal directions; will throw a `MoveError::InvalidPieceType(...)` if a knight's move direction is used.
/// If so, returns a bitmask referring to the defensive piece in the geometric pin, else `Ok(0)`
fn check_for_pin(
    king_position: u64,
    direction: ChessDirection,
    attacking_pieces: u64,
    defending_pieces: u64,
) -> Result<u64, MoveError> {
    use ChessDirection::*;
    // first we confirm that the direction is good
    match direction {
        Up | UpRight | Right | DownRight | Down | DownLeft | Left | UpLeft => {}
        _ => {
            return Err(MoveError::InvalidDirection(
                "check_for_pin".into(),
                "a cardinal or diagonal direction".into(),
                format!("{:?}", direction),
            ))
        }
    }
    // will represent the next square to check
    let mut next_position: u64 = king_position.shift_move(direction);
    // tracks the position of the first defensive piece found
    let mut pinned_piece_candidate: u64 = 0;

    // i.e. there is some valid next move in the direction
    while next_position != 0 {
        let pinned_piece_found: bool = pinned_piece_candidate != 0;

        if !pinned_piece_found {
            // we are looking for a defensive piece to start the pin
            if next_position & defending_pieces != 0 {
                pinned_piece_candidate = next_position;
            } else if next_position & attacking_pieces != 0 {
                // if we find an attacking piece before we find a defending piece, then it is not a valid pin
                return Ok(0);
            }
            // if we don't find anything, keep looking
        } else {
            // we have our defending piece, so we are looking for an attacking piece
            if next_position & attacking_pieces != 0 {
                // its a valid pin!
                return Ok(pinned_piece_candidate);
            } else if next_position & defending_pieces != 0 {
                // we run into a second defending piece before an attacking piece, therefore not a valid pin
                return Ok(0);
            }
            // if we don't find anything, keep looking
        }

        // if we have not returned early, then we can move on to checking the next position
        next_position = next_position.shift_move(direction);
    }

    // if there are no more valid moves and we have not returned a valid pin yet, then there is no valid pin
    Ok(0)
}

#[cfg(test)]
mod tests {
    mod check_for_pin_tests {
        use crate::chess_state::{
            coordinates::{XCoordinate, YCoordinate},
            moves::{
                calculate_moves::pinned_to_king::check_for_pin, chess_move::ChessDirection::*,
            },
        };

        #[test]
        fn returns_0_when_no_pieces() {
            // arrange
            let directions = vec![Up, UpRight, Right, DownRight, Down, DownLeft, Left, UpLeft];
            // act
            let pins: Vec<_> = directions
                .iter()
                .map(|&dir| check_for_pin(0, dir, 0, 0))
                .collect();

            // assert
            assert!(pins.iter().all(|pin| pin.is_ok()));
            assert!(pins.into_iter().all(|pin| pin.expect("is ok") == 0))
        }

        #[test]
        fn returns_correct_error_when_given_invalid_direction() {
            // arrange
            let directions = vec![
                KnightOne,
                KnightTwo,
                KnightFour,
                KnightFive,
                KnightSeven,
                KnightEight,
                KnightTen,
                KnightEleven,
            ];

            // act
            let outputs: Vec<_> = directions
                .iter()
                .map(|&dir| check_for_pin(0, dir, 0, 0))
                .collect();

            // assert
            assert!(outputs.iter().all(|res| res.is_err()));
        }

        #[test]
        fn finds_pin_when_in_simple_valid_pin_stituation() {
            use XCoordinate::*;
            use YCoordinate::*;

            // arrange
            let king_position = (A as u64) & (One as u64);
            let pawn_position = (A as u64) & (Two as u64);
            let enemy_rook_position = (A as u64) & (Eight as u64);
            let expected_pin_position = pawn_position;

            // act
            let pin = check_for_pin(king_position, Up, enemy_rook_position, pawn_position);

            // assert
            assert!(pin.is_ok());
            assert_eq!(pin.unwrap(), expected_pin_position);
        }

        #[test]
        fn finds_pin_when_in_noisy_valid_pin_stituation() {
            use XCoordinate::*;
            use YCoordinate::*;

            // arrange
            // king is in corner
            let king_position = (A as u64) & (One as u64);
            // full row of pawns
            let pawn_position = Two as u64;
            let enemy_rook_position = (A as u64) & (Eight as u64) | (H as u64) & (Eight as u64);
            let expected_pin_position = (A as u64) & (Two as u64);

            // act
            let pin = check_for_pin(king_position, Up, enemy_rook_position, pawn_position);

            // assert
            assert!(pin.is_ok());
            assert_eq!(pin.unwrap(), expected_pin_position);
        }

        #[test]
        fn finds_no_pin_when_in_invalid_pin_stituation() {
            use XCoordinate::*;
            use YCoordinate::*;

            // arrange
            // king is in corner
            let king_position = (A as u64) & (One as u64);
            // full row of pawns
            let pawn_position = Two as u64;
            let enemy_rook_position = (H as u64) & (Eight as u64);

            // act
            let pin = check_for_pin(king_position, Up, enemy_rook_position, pawn_position);

            // assert
            assert!(pin.is_ok());
            assert_eq!(pin.unwrap(), 0);
        }

        #[test]
        fn finds_diagonal_pin_when_in_noisy_valid_pin_stituation() {
            use XCoordinate::*;
            use YCoordinate::*;

            // arrange
            // king is in corner
            let king_position = (A as u64) & (One as u64);
            // full row of pawns
            let pawn_position = Two as u64;
            let enemy_bishop_position = (A as u64) & (Eight as u64) | (H as u64) & (Eight as u64);
            // pin is on the long diagonal, so should be the B pawn
            let expected_pin_position = (B as u64) & (Two as u64);

            // act
            let pin = check_for_pin(king_position, UpRight, enemy_bishop_position, pawn_position);

            // assert
            assert!(pin.is_ok());
            assert_eq!(pin.unwrap(), expected_pin_position);
        }
    }

    mod get_pieces_pinned_to_king {
        use crate::chess_state::{
            board_bitmask::BoardBitmasks,
            coordinates::{XCoordinate, YCoordinate},
        };

        #[test]
        fn can_detect_pinned_pieces_when_in_simple_pin() {
            use XCoordinate::*;
            use YCoordinate::*;

            // arrange
            // set up a simple pin - Black bishop onto pawn onto king
            let game_board = BoardBitmasks {
                all_pieces: ((E as u64 & Four as u64)
                    | (F as u64 & Five as u64)
                    | (G as u64 & Six as u64))
                    .into(),
                white_pieces: ((E as u64 & Four as u64) | (F as u64 & Five as u64)).into(),
                white_pawns: (F as u64 & Five as u64).into(),
                white_knights: 0.into(),
                white_bishops: 0.into(),
                white_rooks: 0.into(),
                white_queens: 0.into(),
                white_kings: (E as u64 & Four as u64).into(),
                black_pieces: (G as u64 & Six as u64).into(),
                black_pawns: 0.into(),
                black_knights: 0.into(),
                black_bishops: (G as u64 & Six as u64).into(),
                black_rooks: 0.into(),
                black_queens: 0.into(),
                black_kings: 0.into(),
            };
            let expected_pin = F as u64 & Five as u64;

            // act
            let pin = game_board.get_pieces_pinned_to_king(true);

            // assert
            assert_eq!(pin, expected_pin)
        }
    }
}
