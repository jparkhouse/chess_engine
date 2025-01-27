pub(crate) trait ChessShiftMove {
    fn shift_move(self, direction: ChessDirection) -> Self;
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum ChessDirection {
    // single step king moves
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,

    // knight moves based on clock
    // up, up, right
    KnightOne,
    // up, right, right
    KnightTwo,
    // down, right, right
    KnightFour,
    // down, down, right
    KnightFive,
    // down, down, left
    KnightSeven,
    // down, left, left
    KnightEight,
    // up, left, left
    KnightTen,
    // up, up, left
    KnightEleven,
}

impl ChessShiftMove for u64 {
    #[inline]
    fn shift_move(self, direction: ChessDirection) -> Self {
        use ChessDirection::*;
        // to help with safety, first filter out any pieces that cannot make that move
        let valid_pieces = self & crate::chess_state::moves::shared::get_valid_space(direction);
        match direction {
            Up => valid_pieces << 8,
            Right => valid_pieces >> 1,
            Down => valid_pieces >> 8,
            Left => valid_pieces << 1,

            UpRight => valid_pieces.shift_move(Up).shift_move(Right),
            DownRight => valid_pieces.shift_move(Down).shift_move(Right),
            DownLeft => valid_pieces.shift_move(Down).shift_move(Left),
            UpLeft => valid_pieces.shift_move(Up).shift_move(Left),

            KnightOne => valid_pieces.shift_move(Up).shift_move(Up).shift_move(Right),
            KnightTwo => valid_pieces
                .shift_move(Up)
                .shift_move(Right)
                .shift_move(Right),
            KnightFour => valid_pieces
                .shift_move(Down)
                .shift_move(Right)
                .shift_move(Right),
            KnightFive => valid_pieces
                .shift_move(Down)
                .shift_move(Down)
                .shift_move(Right),
            KnightSeven => valid_pieces
                .shift_move(Down)
                .shift_move(Down)
                .shift_move(Left),
            KnightEight => valid_pieces
                .shift_move(Down)
                .shift_move(Left)
                .shift_move(Left),
            KnightTen => valid_pieces
                .shift_move(Up)
                .shift_move(Left)
                .shift_move(Left),
            KnightEleven => valid_pieces.shift_move(Up).shift_move(Up).shift_move(Left),
        }
    }
}

#[cfg(test)]
mod tests {
    mod chess_move_for_u64 {
        // TODO: add validation on excluded pieces
        use crate::chess_state::{
            coordinates::{XCoordinate::*, YCoordinate::*},
            moves::chess_move::{ChessDirection::*, ChessShiftMove},
        };

        #[test]
        fn moves_up_when_directed_up() {
            // arrange
            let start = (A as u64) & (Two as u64);
            let expected_output = (A as u64) & (Three as u64);
            // act
            let output = start.shift_move(Up);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn moves_left_when_directed_left() {
            // arrange
            let start = (B as u64) & (Two as u64);
            let expected_output = (A as u64) & (Two as u64);
            // act
            let output = start.shift_move(Left);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn moves_right_when_directed_right() {
            // arrange
            let start = (A as u64) & (Two as u64);
            let expected_output = (B as u64) & (Two as u64);
            // act
            let output = start.shift_move(Right);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn moves_down_when_directed_down() {
            // arrange
            let start = (A as u64) & (Three as u64);
            let expected_output = (A as u64) & (Two as u64);
            // act
            let output = start.shift_move(Down);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn moves_up_right_when_directed_up_right() {
            // arrange
            let start = (A as u64) & (Two as u64);
            let expected_output = (B as u64) & (Three as u64);
            // act
            let output = start.shift_move(UpRight);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn moves_up_left_when_directed_up_left() {
            // arrange
            let start = (C as u64) & (Three as u64);
            let expected_output = (B as u64) & (Four as u64);
            // act
            let output = start.shift_move(UpLeft);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn moves_down_right_when_directed_down_right() {
            // arrange
            let start = (F as u64) & (Five as u64);
            let expected_output = (G as u64) & (Four as u64);
            // act
            let output = start.shift_move(DownRight);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn moves_down_left_when_directed_down_left() {
            // arrange
            let start = (C as u64) & (Three as u64);
            let expected_output = (B as u64) & (Two as u64);
            // act
            let output = start.shift_move(DownLeft);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn knight_move_one_when_directed() {
            // arrange
            let start = (D as u64) & (Four as u64);
            let expected_output = (E as u64) & (Six as u64);
            // act
            let output = start.shift_move(KnightOne);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn knight_move_two_when_directed() {
            // arrange
            let start = (D as u64) & (Four as u64);
            let expected_output = (F as u64) & (Five as u64);
            // act
            let output = start.shift_move(KnightTwo);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn knight_move_four_when_directed() {
            // arrange
            let start = (D as u64) & (Four as u64);
            let expected_output = (F as u64) & (Three as u64);
            // act
            let output = start.shift_move(KnightFour);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn knight_move_five_when_directed() {
            // arrange
            let start = (D as u64) & (Four as u64);
            let expected_output = (E as u64) & (Two as u64);
            // act
            let output = start.shift_move(KnightFive);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn knight_move_seven_when_directed() {
            // arrange
            let start = (D as u64) & (Four as u64);
            let expected_output = (C as u64) & (Two as u64);
            // act
            let output = start.shift_move(KnightSeven);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn knight_move_eight_when_directed() {
            // arrange
            let start = (D as u64) & (Four as u64);
            let expected_output = (B as u64) & (Three as u64);
            // act
            let output = start.shift_move(KnightEight);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn knight_move_ten_when_directed() {
            // arrange
            let start = (D as u64) & (Four as u64);
            let expected_output = (B as u64) & (Five as u64);
            // act
            let output = start.shift_move(KnightTen);
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn knight_move_eleven_when_directed() {
            // arrange
            let start = (D as u64) & (Four as u64);
            let expected_output = (C as u64) & (Six as u64);
            // act
            let output = start.shift_move(KnightEleven);
            // assert
            assert_eq!(output, expected_output)
        }
    }
}
