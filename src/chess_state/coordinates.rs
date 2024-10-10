use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum CoordinateError {
    #[error("char {0} does not convert to valid x coordinate in range a-h")]
    XCoordinateFromInvalidChar(char),

    #[error("char {0} does not convert to valid y coordinate in range 1-8")]
    YCoordinateFromInvalidChar(char),

    #[error("String {0} does not convert to valid xy coordinates")]
    XYCoordinatesFromInvalidStr(String),

    #[error("String {0} is not of length 2 to be valid xy coordinates")]
    XYCoordinatesFromInvalidLengthStr(String),

    #[error("Bitmask {0} does not convert to valid x coordinates")]
    XCoordinateFromInvalidBitmask(u64),

    #[error("Bitmask {0} does not convert to valid y coordinates")]
    YCoordinateFromInvalidBitmask(u64),

    #[error("Bitmask {0} contains no set bit, relating to no position")]
    XYCoordinatesFromEmptyBitmask(u64),

    #[error("Bitmask {0} contains more than one set bit, relating to multiple positions")]
    XYCoordinatesFromMultibitBitmask(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum XCoordinate {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum YCoordinate {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

pub(crate) trait CoordinateConversion<T>: Sized {
    type Error;

    fn from_value(value: T) -> Result<Self, Self::Error>;
    fn to_value(self) -> T;
}

impl CoordinateConversion<char> for XCoordinate {
    type Error = CoordinateError;

    fn from_value(value: char) -> Result<Self, Self::Error> {
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

    fn from_value(value: char) -> Result<Self, Self::Error> {
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

    fn from_value(value: u64) -> Result<Self, Self::Error> {
        use CoordinateError::*;
        use XCoordinate::*;

        let variants = [A, B, C, D, E, F, G, H];

        return variants
            .iter()
            .find(|&&coord| CoordinateConversion::<u64>::to_value(coord) & value > 0)
            .copied()
            .ok_or(XCoordinateFromInvalidBitmask(value));
    }

    fn to_value(self) -> u64 {
        let shift = self as u64;
        let mut bitmask = 0 as u64;
        for row in 0..8 {
            bitmask |= 1u64 << (shift + row * 8)
        }
        return bitmask;
    }
}

impl CoordinateConversion<u64> for YCoordinate {
    type Error = CoordinateError;

    fn from_value(value: u64) -> Result<Self, Self::Error> {
        use CoordinateError::*;
        use YCoordinate::*;

        let variants = [One, Two, Three, Four, Five, Six, Seven, Eight];

        return variants
            .iter()
            .find(|&&coord| CoordinateConversion::<u64>::to_value(coord) & value > 0)
            .copied()
            .ok_or(YCoordinateFromInvalidBitmask(value));
    }

    fn to_value(self) -> u64 {
        // we take a row, which is 0xFF (8 consecutive on bits)
        // then we figure out how much to shift it by
        // One = 0u8, so requires no shifting
        // Two = 1u8, so requires shifting by 8
        // etc etc
        let shift = (self as u64) * 8;
        return 0xFFu64 << shift;
    }
}
