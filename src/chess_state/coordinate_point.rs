use crate::{chess_state::coordinates::{
    CoordinateConversion,
    CoordinateError,
    XCoordinate, 
    YCoordinate
}, shared::has_one_bit_set};
use std::fmt;


#[derive(Debug, Clone, Copy)]
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

        let chars: [char; 2] = [
            input.chars().next().expect("Length is 2"),
            input.chars().next().expect("Length is 2"),
        ];

        let x_enum: XCoordinate = CoordinateConversion::<char>::try_from_value(chars[0])?;
        let y_enum: YCoordinate = CoordinateConversion::<char>::try_from_value(chars[1])?;

        return Ok(Self {
            x: x_enum,
            y: y_enum,
        })
    }

    pub(crate) fn from_bitmask(bitmask: u64) -> Result<Self, CoordinateError> {
        use CoordinateError::{
            XYCoordinatesFromEmptyBitmask,
            XYCoordinatesFromMultibitBitmask
        };

        if has_one_bit_set(bitmask) {
            let x_enum: XCoordinate = CoordinateConversion::<u64>::try_from_value(bitmask)?;
            let y_enum: YCoordinate = CoordinateConversion::<u64>::try_from_value(bitmask)?;
            return Ok(Self {
                x: x_enum,
                y: y_enum,
            })
        } else if bitmask != 0 {
            return Err(XYCoordinatesFromMultibitBitmask(bitmask))
        } else {
            return Err(XYCoordinatesFromEmptyBitmask(bitmask))
        }
    }

    pub(crate) fn to_bitmask(&self) -> u64 {
        let all_x = CoordinateConversion::<u64>::to_value(self.x);
        let all_y = CoordinateConversion::<u64>::to_value(self.y);
        // the only intersecting point of x and y is the pair (x, y)
        return all_x & all_y
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
