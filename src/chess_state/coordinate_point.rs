use crate::{
    chess_state::coordinates::{CoordinateConversion, CoordinateError, XCoordinate, YCoordinate},
    shared::has_one_bit_set,
};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CoordinatePosition {
    pub x: XCoordinate,
    pub y: YCoordinate,
}

impl CoordinatePosition {
    pub(crate) fn from_str(input: &str) -> Result<Self, CoordinateError> {
        use CoordinateError::*;

        if input.len() != 2 {
            return Err(XYCoordinatesFromInvalidLengthStr(input.to_string()));
        }

        let mut iter_over_str = input.chars();
        let x_char = iter_over_str.next().expect("Length is 2");
        let y_char = iter_over_str.next().expect("Length is 2");

        let x_enum: XCoordinate = CoordinateConversion::<char>::try_from_value(x_char)?;
        let y_enum: YCoordinate = CoordinateConversion::<char>::try_from_value(y_char)?;

        Ok(Self {
            x: x_enum,
            y: y_enum,
        })
    }

    pub(crate) fn from_bitmask(bitmask: u64) -> Result<Self, CoordinateError> {
        use CoordinateError::{XYCoordinatesFromEmptyBitmask, XYCoordinatesFromMultiBitBitmask};

        if has_one_bit_set(bitmask) {
            let x_enum: XCoordinate = CoordinateConversion::<u64>::try_from_value(bitmask)?;
            let y_enum: YCoordinate = CoordinateConversion::<u64>::try_from_value(bitmask)?;
            Ok(Self {
                x: x_enum,
                y: y_enum,
            })
        } else if bitmask != 0 {
            Err(XYCoordinatesFromMultiBitBitmask(bitmask))
        } else {
            Err(XYCoordinatesFromEmptyBitmask(bitmask))
        }
    }

    pub(crate) fn to_bitmask(self) -> u64 {
        let all_x = CoordinateConversion::<u64>::to_value(self.x);
        let all_y = CoordinateConversion::<u64>::to_value(self.y);
        // the only intersecting point of x and y is the pair (x, y)
        all_x & all_y
    }
}

// Implement the Display trait for Point
impl fmt::Display for CoordinatePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x_char = CoordinateConversion::<char>::to_value(self.x);
        let y_char = CoordinateConversion::<char>::to_value(self.y);
        write!(f, "{}{}", x_char, y_char)
    }
}

#[cfg(test)]
mod tests {
    mod unit_tests {
        mod from_str {
            use crate::chess_state::{
                coordinate_point::CoordinatePosition,
                coordinates::{CoordinateError, XCoordinate, YCoordinate},
            };

            #[test]
            fn returns_correct_coordinate_position_when_given_valid_str() {
                // arrange
                let string_1 = String::from("a1");
                let string_2 = String::from("h8");
                let string_3 = String::from("d4");

                let string_1_coord = CoordinatePosition {
                    x: XCoordinate::A,
                    y: YCoordinate::One,
                };
                let string_2_coord = CoordinatePosition {
                    x: XCoordinate::H,
                    y: YCoordinate::Eight,
                };
                let string_3_coord = CoordinatePosition {
                    x: XCoordinate::D,
                    y: YCoordinate::Four,
                };

                // act
                let output: Vec<CoordinatePosition> = [string_1, string_2, string_3]
                    .iter()
                    .map(|s| CoordinatePosition::from_str(s).expect("should be valid str"))
                    .collect();

                // assert
                assert_eq!(output, vec![string_1_coord, string_2_coord, string_3_coord])
            }

            #[test]
            fn returns_error_when_given_string_not_of_length_2() {
                use CoordinateError::XYCoordinatesFromInvalidLengthStr;
                // arrange
                let strings = vec!["a", "a12", "b13", "c521"];
                let expected_output: Vec<CoordinateError> = strings.iter().map(|&s| XYCoordinatesFromInvalidLengthStr(s.to_string())).collect();
                // act
                let output: Vec<CoordinateError> = strings.into_iter().map(|s| CoordinatePosition::from_str(s).expect_err("invalid string lengths should error")).collect();
                // assert
                assert_eq!(output, expected_output)
            }

            #[test]
            fn returns_error_when_given_invalid_characters() {
                use CoordinateError::{XCoordinateFromInvalidChar, YCoordinateFromInvalidChar};
                // arrange
                let strings = vec!["z9", "b@", "89"];
                // both z and 9 are out of range
                // b is fine, but '@' is invalid
                // 8 is not a valid x coordinate
                let expected_output: Vec<CoordinateError> = vec![
                    XCoordinateFromInvalidChar('z'),
                    YCoordinateFromInvalidChar('@'),
                    XCoordinateFromInvalidChar('8')
                ];
                // act
                let output: Vec<CoordinateError> = strings.into_iter().map(|s| CoordinatePosition::from_str(s).expect_err("expecting invalid char errors")).collect();
                // assert
                assert_eq!(output, expected_output)
            }
        }

        mod from_bitmask {
            use crate::chess_state::{coordinate_point::CoordinatePosition, coordinates::{CoordinateError, XCoordinate, YCoordinate}};

            #[test]
            fn returns_valid_coordinate_position_when_given_valid_bitmask() {
                // arrange
                let valid_bitmask = (XCoordinate::A as u64) & (YCoordinate::Two as u64);
                let expected_output = CoordinatePosition { x: XCoordinate::A, y: YCoordinate::Two};
                // act
                let output = CoordinatePosition::from_bitmask(valid_bitmask).expect("uses valid bitmask");
                // assert
                assert_eq!(output, expected_output)
            }

            #[test]
            fn returns_correct_error_when_given_empty_bitmask() {
                // arrange
                let empty_bitmask: u64 = 0;
                let expected_output = CoordinateError::XYCoordinatesFromEmptyBitmask(empty_bitmask);
                // act
                let output = CoordinatePosition::from_bitmask(empty_bitmask).expect_err("uses invalid bitmask");
                // assert
                assert_eq!(output, expected_output)
            }

            #[test]
            fn returns_correct_error_when_given_multi_bit_bitmask() {
                // arrange
                // a3 and d3 set
                let multi_bit_bitmask = ((XCoordinate::A as u64) | (XCoordinate::D as u64)) & (YCoordinate::Three as u64);
                let expected_output = CoordinateError::XYCoordinatesFromMultiBitBitmask(multi_bit_bitmask);
                // act
                let output = CoordinatePosition::from_bitmask(multi_bit_bitmask).expect_err("uses multi bit bitmask");
                // assert
                assert_eq!(output, expected_output)
            }
        }

        mod to_bitmask {
            use crate::chess_state::{coordinate_point::CoordinatePosition, coordinates::{XCoordinate, YCoordinate}};

            #[test]
            fn returns_valid_bitmask_when_converting_from_coordinate_position() {
                // arrange
                let coord = CoordinatePosition { x: XCoordinate::E, y: YCoordinate::Four};
                let expected_output = (XCoordinate::E as u64) & (YCoordinate::Four as u64);
                // act
                let output = coord.to_bitmask();
                // assert
                assert_eq!(output, expected_output)
            }
        }

        mod display {
            use crate::chess_state::{coordinate_point::CoordinatePosition, coordinates::{XCoordinate, YCoordinate}};

            #[test]
            fn generates_valid_string_when_using_to_string_method_from_display_trait() {
                // arrange
                let coords = vec![
                    CoordinatePosition { x: XCoordinate::A , y: YCoordinate::One },
                    CoordinatePosition { x: XCoordinate::C , y: YCoordinate::Three },
                    CoordinatePosition { x: XCoordinate::F , y: YCoordinate::Five },
                    CoordinatePosition { x: XCoordinate::G , y: YCoordinate::Seven }
                ];
                let expected_output: Vec<String> = vec![
                    String::from("a1"),
                    String::from("c3"),
                    String::from("f5"),
                    String::from("g7")
                ];
                // act
                let output: Vec<String> = coords.into_iter().map(|c| c.to_string()).collect();
                // assert
                assert_eq!(output, expected_output)
            }
        }
    }

    mod round_trips {
        use crate::chess_state::{coordinate_point::CoordinatePosition, coordinates::{XCoordinate, YCoordinate}};

        #[test]
        fn completes_round_trip_when_to_and_from_str() {
            // arrange
            let str = "a4";
            // act
            let coord = CoordinatePosition::from_str(str).expect("uses valid string");
            let output = coord.to_string();
            // assert
            assert_eq!(output.as_str(), str)
        }

        #[test]
        fn completes_round_trip_when_to_and_from_bitmask() {
            // arrange
            let valid_bitmask = (XCoordinate::D as u64) & (YCoordinate::Six as u64);
            // act
            let coord = CoordinatePosition::from_bitmask(valid_bitmask).expect("uses valid bitmask");
            let output = coord.to_bitmask();
            // assert
            assert_eq!(output, valid_bitmask)
        }
    }
}
