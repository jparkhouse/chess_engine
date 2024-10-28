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

    #[error("Bitmask {0} converts to more than one X coordinate")]
    XCoordinateFromInvalidBitmask(u64),

    #[error("Bitmask {0} converts to more than one Y coordinate")]
    YCoordinateFromInvalidBitmask(u64),

    #[error("Bitmask {0} contains no set bit, relating to no position")]
    XYCoordinatesFromEmptyBitmask(u64),

    #[error("Bitmask {0} contains more than one set bit, relating to multiple positions")]
    XYCoordinatesFromMultibitBitmask(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub(crate) enum XCoordinate {
    A = 0xFF << 0,
    B = 0xFF << 1,
    C = 0xFF << 2,
    D = 0xFF << 3,
    E = 0xFF << 4,
    F = 0xFF << 5,
    G = 0xFF << 6,
    H = 0xFF << 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
pub(crate) enum YCoordinate {
    One = 0x01_01_01_01_01_01_01_01 << 7,
    Two = 0x01_01_01_01_01_01_01_01 << 6,
    Three = 0x01_01_01_01_01_01_01_01 << 5,
    Four = 0x01_01_01_01_01_01_01_01 << 4,
    Five = 0x01_01_01_01_01_01_01_01 << 3,
    Six = 0x01_01_01_01_01_01_01_01 << 2,
    Seven = 0x01_01_01_01_01_01_01_01 << 1,
    Eight = 0x01_01_01_01_01_01_01_01 << 0,
}

pub(crate) trait CoordinateConversion<T>: Sized {
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
            return Err(CoordinateError::XCoordinateFromInvalidBitmask(value))
        }
        return Ok(output)
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
            return Err(CoordinateError::YCoordinateFromInvalidBitmask(value))
        }
        return Ok(output)
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