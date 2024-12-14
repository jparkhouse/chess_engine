pub(crate) trait ChessFlip {
    fn flip_horizontal(self) -> Self;
    fn flip_vertical(self) -> Self;
    fn flip(self) -> Self;
}

impl ChessFlip for u64 {
    fn flip_horizontal(self) -> Self {
        fn invert_byte(byte: u8) -> u8 {
            let mut new_byte: u8 = 0;
            if byte > 0 {
                (0..8).for_each(|bit_no| {
                    let bit = byte & (1 << (7 - bit_no));
                    if bit > 0 {
                        new_byte |= 1 << bit_no;
                    }
                });
            }
            new_byte
        }

        let new_bytes = self.to_be_bytes().map(invert_byte);
        u64::from_be_bytes(new_bytes)
    }

    fn flip_vertical(self) -> Self {
        self.swap_bytes()
    }

    fn flip(self) -> Self {
        self.flip_horizontal().flip_vertical()
    }
}

#[cfg(test)]
mod tests {
    mod chess_flip_for_u64 {
        use crate::chess_state::{
            coordinates::{XCoordinate::*, YCoordinate::*},
            moves::chess_flip::ChessFlip,
        };

        #[test]
        fn bits_swap_correctly_when_swapped_horizontally() {
            // arrange
            let test_input = ((A as u64) & (One as u64)) | ((B as u64) & (One as u64));
            let expected_output = ((G as u64) & (One as u64)) | ((H as u64) & (One as u64));

            // act
            let output = test_input.flip_horizontal();

            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn bits_swap_correctly_when_board_flipped() {
            // arrange
            let test_input = ((A as u64) & (One as u64))
                | ((B as u64) & (One as u64))
                | ((C as u64) & (Two as u64))
                | ((D as u64) & (Three as u64));
            // in respective order
            let expected_output = ((H as u64) & (Eight as u64))
                | ((G as u64) & (Eight as u64))
                | ((F as u64) & (Seven as u64))
                | ((E as u64) & (Six as u64));

            // act
            let output = test_input.flip();

            // assert
            assert_eq!(output, expected_output)
        }
    }
}
