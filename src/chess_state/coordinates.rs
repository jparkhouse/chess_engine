use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum CoordinateError {
    #[error("char {0} does not convert to valid x coordinate in range a-h")]
    XCoordinateFromInvalidChar(char),

    #[error("char {0} does not convert to valid y coordinate in range 1-8")]
    YCoordinateFromInvalidChar(char),

    #[error("String {0} does not convert to valid xy coordinates")]
    XYCoordinatesFromInvalidStr(String),

    #[error("String {0} is not of length 2 to be valid xy coordinates")]
    XYCoordinatesFromInvalidLengthStr(String),

    #[error("Bitmask {0} converts to more than one X coordinate")]
    XCoordinateFromInvalidBitmask(u64),

    #[error("Bitmask {0} converts to more than one Y coordinate")]
    YCoordinateFromInvalidBitmask(u64),

    #[error("Bitmask {0} contains no set bit, relating to no position")]
    XYCoordinatesFromEmptyBitmask(u64),

    #[error("Bitmask {0} contains more than one set bit, relating to multiple positions")]
    XYCoordinatesFromMultiBitBitmask(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub(crate) enum XCoordinate {
    A = 0x01_01_01_01_01_01_01_01 << 7,
    B = 0x01_01_01_01_01_01_01_01 << 6,
    C = 0x01_01_01_01_01_01_01_01 << 5,
    D = 0x01_01_01_01_01_01_01_01 << 4,
    E = 0x01_01_01_01_01_01_01_01 << 3,
    F = 0x01_01_01_01_01_01_01_01 << 2,
    G = 0x01_01_01_01_01_01_01_01 << 1,
    H = 0x01_01_01_01_01_01_01_01,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub(crate) enum YCoordinate {
    One =   0xFF,
    Two =   0xFF << 8,
    Three = 0xFF << (2 * 8),
    Four =  0xFF << (3 * 8),
    Five =  0xFF << (4 * 8),
    Six =   0xFF << (5 * 8),
    Seven = 0xFF << (6 * 8),
    Eight = 0xFF << (7 * 8),
}

pub trait CoordinateConversion<T>: Sized {
    type Error;

    fn try_from_value(value: T) -> Result<Self, Self::Error>;
    fn to_value(self) -> T;
}

impl CoordinateConversion<char> for XCoordinate {
    type Error = CoordinateError;

    fn try_from_value(value: char) -> Result<Self, Self::Error> {
        use CoordinateError::*;
        use XCoordinate::*;

        match value {
            'a' | 'A' => Ok(A),
            'b' | 'B' => Ok(B),
            'c' | 'C' => Ok(C),
            'd' | 'D' => Ok(D),
            'e' | 'E' => Ok(E),
            'f' | 'F' => Ok(F),
            'g' | 'G' => Ok(G),
            'h' | 'H' => Ok(H),
            _ => Err(XCoordinateFromInvalidChar(value)),
        }
    }

    fn to_value(self) -> char {
        use XCoordinate::*;
        match self {
            A => 'a',
            B => 'b',
            C => 'c',
            D => 'd',
            E => 'e',
            F => 'f',
            G => 'g',
            H => 'h',
        }
    }
}

impl CoordinateConversion<char> for YCoordinate {
    type Error = CoordinateError;

    fn try_from_value(value: char) -> Result<Self, Self::Error> {
        use CoordinateError::*;
        use YCoordinate::*;

        match value {
            '1' => Ok(One),
            '2' => Ok(Two),
            '3' => Ok(Three),
            '4' => Ok(Four),
            '5' => Ok(Five),
            '6' => Ok(Six),
            '7' => Ok(Seven),
            '8' => Ok(Eight),
            _ => Err(YCoordinateFromInvalidChar(value)),
        }
    }

    fn to_value(self) -> char {
        use YCoordinate::*;

        match self {
            One => '1',
            Two => '2',
            Three => '3',
            Four => '4',
            Five => '5',
            Six => '6',
            Seven => '7',
            Eight => '8',
        }
    }
}

impl CoordinateConversion<u64> for XCoordinate {
    type Error = CoordinateError;

    fn try_from_value(value: u64) -> Result<Self, Self::Error> {
        use XCoordinate::*;
        let mut counter = 0;
        let mut output = XCoordinate::A;
        for i in [A, B, C, D, E, F, G, H] {
            if value & (i as u64) > 0 {
                counter += 1;
                output = i;
            }
        }
        if counter > 1 {
            return Err(CoordinateError::XCoordinateFromInvalidBitmask(value));
        }
        Ok(output)
    }

    fn to_value(self) -> u64 {
        self as u64
    }
}

impl CoordinateConversion<u64> for YCoordinate {
    type Error = CoordinateError;

    fn try_from_value(value: u64) -> Result<Self, Self::Error> {
        use YCoordinate::*;
        let mut counter = 0;
        let mut output = YCoordinate::One;
        for i in [One, Two, Three, Four, Five, Six, Seven, Eight] {
            if value & (i as u64) > 0 {
                counter += 1;
                output = i;
            }
        }
        if counter > 1 {
            return Err(CoordinateError::YCoordinateFromInvalidBitmask(value));
        }
        Ok(output)
    }

    fn to_value(self) -> u64 {
        self as u64
    }
}

impl From<XCoordinate> for u64 {
    fn from(value: XCoordinate) -> Self {
        value as u64
    }
}

impl From<YCoordinate> for u64 {
    fn from(value: YCoordinate) -> Self {
        value as u64
    }
}

#[cfg(test)]
mod coordinates_tests {
    mod x_coordinate_conversion_to_char {
        use crate::chess_state::coordinates::{
            CoordinateConversion, CoordinateError, XCoordinate, XCoordinate::*,
        };

        #[test]
        fn converts_enum_value_correctly_when_given_valid_char() {
            // arrange
            let valid_lower_case: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
            let valid_upper_case: [char; 8] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
            let expected_output: Vec<XCoordinate> = vec![A, B, C, D, E, F, G, H];
            // act
            let enums_from_lower_case: Vec<XCoordinate> = valid_lower_case
                .iter()
                .map(|&ch| {
                    <XCoordinate as CoordinateConversion<char>>::try_from_value(ch)
                        .expect("valid lower case char")
                })
                .collect();
            let enums_from_upper_case: Vec<XCoordinate> = valid_upper_case
                .iter()
                .map(|&ch| {
                    <XCoordinate as CoordinateConversion<char>>::try_from_value(ch)
                        .expect("valid upper case char")
                })
                .collect();
            // assert
            assert_eq!(enums_from_lower_case, expected_output);
            assert_eq!(enums_from_upper_case, expected_output);
        }

        #[test]
        fn returns_correct_error_when_given_invalid_char() {
            // arrange
            let invalid_chars = ['v', '3', '"', '@', 't', 'Y'];
            let expected_output: Vec<CoordinateError> = invalid_chars
                .iter()
                .map(|&ch| CoordinateError::XCoordinateFromInvalidChar(ch))
                .collect();
            // act
            let output: Vec<CoordinateError> = invalid_chars
                .iter()
                .map(|&ch| {
                    <XCoordinate as CoordinateConversion<char>>::try_from_value(ch)
                        .expect_err("invalid char")
                })
                .collect();
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn returns_correct_char_when_converted_from_enum() {
            // arrange
            let enums = [A, B, C, D, E, F, G, H];
            let expected_output = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
            // act
            let output: Vec<char> = enums
                .iter()
                .map(|&e| CoordinateConversion::<char>::to_value(e))
                .collect();
            // assert
            assert_eq!(output, expected_output)
        }
    }
    mod y_coordinate_conversion_to_char {
        use crate::chess_state::coordinates::{
            CoordinateConversion, CoordinateError, YCoordinate, YCoordinate::*,
        };

        #[test]
        fn converts_enum_value_correctly_when_given_valid_char() {
            // arrange
            let valid_numbers: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];
            let expected_output: Vec<YCoordinate> =
                vec![One, Two, Three, Four, Five, Six, Seven, Eight];
            // act
            let enums_from_valid_numbers: Vec<YCoordinate> = valid_numbers
                .iter()
                .map(|&ch| {
                    <YCoordinate as CoordinateConversion<char>>::try_from_value(ch)
                        .expect("valid number char")
                })
                .collect();
            // assert
            assert_eq!(enums_from_valid_numbers, expected_output);
        }

        #[test]
        fn returns_correct_error_when_given_invalid_char() {
            // arrange
            let invalid_chars = ['v', 'a', '"', '@', 't', 'Y'];
            let expected_output: Vec<CoordinateError> = invalid_chars
                .iter()
                .map(|&ch| CoordinateError::YCoordinateFromInvalidChar(ch))
                .collect();
            // act
            let output: Vec<CoordinateError> = invalid_chars
                .iter()
                .map(|&ch| {
                    <YCoordinate as CoordinateConversion<char>>::try_from_value(ch)
                        .expect_err("invalid char")
                })
                .collect();
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn returns_correct_char_when_converted_from_enum() {
            // arrange
            let enums = [One, Two, Three, Four, Five, Six, Seven, Eight];
            let expected_output = vec!['1', '2', '3', '4', '5', '6', '7', '8'];
            // act
            let output: Vec<char> = enums
                .iter()
                .map(|&e| CoordinateConversion::<char>::to_value(e))
                .collect();
            // assert
            assert_eq!(output, expected_output)
        }
    }

    mod x_coordinate_conversion_to_u64 {
        use crate::chess_state::coordinates::{
            CoordinateConversion, CoordinateError, XCoordinate, XCoordinate::*,
        };

        #[test]
        fn returns_correct_u64s_when_converting_from_enums() {
            // arrange
            let enums = [A, B, C, D, E, F, G, H];
            let expected_output: Vec<u64> = vec![
                0x01_01_01_01_01_01_01_01 << 7,
                0x01_01_01_01_01_01_01_01 << 6,
                0x01_01_01_01_01_01_01_01 << 5,
                0x01_01_01_01_01_01_01_01 << 4,
                0x01_01_01_01_01_01_01_01 << 3,
                0x01_01_01_01_01_01_01_01 << 2,
                0x01_01_01_01_01_01_01_01 << 1,
                0x01_01_01_01_01_01_01_01 << 0,
            ];
            // act
            let output: Vec<u64> = enums
                .iter()
                .map(|&e| <XCoordinate as CoordinateConversion<u64>>::to_value(e))
                .collect();
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn returns_correct_error_when_given_multi_coordinate_bitflag() {
            // arrange
            let invalid_bitflag: u64 = // both A and B
                0x01_01_01_01_01_01_01_01 << 7 | 0x01_01_01_01_01_01_01_01 << 6;
            // act
            let output =
                <XCoordinate as CoordinateConversion<u64>>::try_from_value(invalid_bitflag);
            // assert
            assert!(output.is_err());
            let err = output.expect_err("must be an error");
            assert_eq!(
                err,
                CoordinateError::XCoordinateFromInvalidBitmask(invalid_bitflag)
            )
        }

        #[test]
        fn returns_correct_enum_values_when_given_valid_u64s() {
            // arrange
            let enums_as_u64s: [u64; 8] = [
                0x01_01_01_01_01_01_01_01 << 7,
                0x01_01_01_01_01_01_01_01 << 6,
                0x01_01_01_01_01_01_01_01 << 5,
                0x01_01_01_01_01_01_01_01 << 4,
                0x01_01_01_01_01_01_01_01 << 3,
                0x01_01_01_01_01_01_01_01 << 2,
                0x01_01_01_01_01_01_01_01 << 1,
                0x01_01_01_01_01_01_01_01 << 0,
            ];
            let expected_output: Vec<XCoordinate> = vec![A, B, C, D, E, F, G, H];
            // act
            let output: Vec<XCoordinate> = enums_as_u64s
                .iter()
                .map(|&u| {
                    <XCoordinate as CoordinateConversion<u64>>::try_from_value(u)
                        .expect("Valid u64 value should produce valid enum")
                })
                .collect();
            // assert
            assert_eq!(output, expected_output)
        }
    }

    mod y_coordinate_conversion_to_u64 {
        use crate::chess_state::coordinates::{
            CoordinateConversion, CoordinateError, YCoordinate, YCoordinate::*,
        };

        #[test]
        fn returns_correct_u64s_when_converting_from_enums() {
            // arrange
            let enums = [One, Two, Three, Four, Five, Six, Seven, Eight];
            let expected_output: Vec<u64> = vec![
                0xFF << 0,
                0xFF << 8,
                0xFF << 16,
                0xFF << 24,
                0xFF << 32,
                0xFF << 40,
                0xFF << 48,
                0xFF << 56,
            ];
            // act
            let output: Vec<u64> = enums
                .iter()
                .map(|&e| <YCoordinate as CoordinateConversion<u64>>::to_value(e))
                .collect();
            // assert
            assert_eq!(output, expected_output)
        }

        #[test]
        fn returns_correct_error_when_given_multi_coordinate_bitflag() {
            // arrange
            let invalid_bitflag: u64 = // both A and B, crossing all YCoordinates
                0x01_01_01_01_01_01_01_01 << 7 | 0x01_01_01_01_01_01_01_01 << 6;
            // act
            let output =
                <YCoordinate as CoordinateConversion<u64>>::try_from_value(invalid_bitflag);
            // assert
            assert!(output.is_err());
            let err = output.expect_err("must be an error");
            assert_eq!(
                err,
                CoordinateError::YCoordinateFromInvalidBitmask(invalid_bitflag)
            )
        }

        #[test]
        fn returns_correct_enum_values_when_given_valid_u64s() {
            // arrange
            let enums_as_u64s: [u64; 8] = [
                0xFF << 0,
                0xFF << 8,
                0xFF << 16,
                0xFF << 24,
                0xFF << 32,
                0xFF << 40,
                0xFF << 48,
                0xFF << 56,
            ];
            let expected_output: Vec<YCoordinate> =
                vec![One, Two, Three, Four, Five, Six, Seven, Eight];
            // act
            let output: Vec<YCoordinate> = enums_as_u64s
                .iter()
                .map(|&u| {
                    <YCoordinate as CoordinateConversion<u64>>::try_from_value(u)
                        .expect("Valid u64 value should produce valid enum")
                })
                .collect();
            // assert
            assert_eq!(output, expected_output)
        }
    }
}
