use std::{
    marker::PhantomData,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, Mul, Not, Shl},
};

#[derive(Debug, Clone, Copy)]
pub(crate) struct Bitmask<T> {
    pub(crate) mask: u64,
    pub(crate) _marker: PhantomData<T>,
}

impl<T> Bitmask<T> {
    pub(crate) fn new() -> Self {
        Self {
            mask: 0,
            _marker: PhantomData,
        }
    }

    pub(crate) fn from_u64(value: u64) -> Self {
        Self {
            mask: value,
            _marker: PhantomData,
        }
    }

    pub(crate) fn to_u64(&self) -> u64 {
        self.mask.clone()
    }
}

impl<T> From<u64> for Bitmask<T> {
    fn from(value: u64) -> Self {
        Self {
            mask: value,
            _marker: PhantomData,
        }
    }
}

impl<T> BitOr for Bitmask<T> {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            mask: self.mask | rhs.mask,
            _marker: PhantomData,
        }
    }
}

impl<T> BitAnd for Bitmask<T> {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self {
            mask: self.mask & rhs.mask,
            _marker: PhantomData,
        }
    }
}

impl<T> Not for Bitmask<T> {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self {
            mask: !self.mask,
            _marker: PhantomData,
        }
    }
}

impl<T> BitOrAssign for Bitmask<T> {
    fn bitor_assign(&mut self, rhs: Self) {
        self.mask |= rhs.mask
    }
}

impl<T> BitAndAssign for Bitmask<T> {
    fn bitand_assign(&mut self, rhs: Self) {
        self.mask &= rhs.mask
    }
}

impl<T> Mul<[f64; 64]> for Bitmask<T> {
    type Output = [f64; 64];

    fn mul(self, rhs: [f64; 64]) -> Self::Output {
        let mut _bitmask = self.mask.clone();
        let mut output: [f64; 64] = [0 as f64; 64]; 
        while _bitmask > 0 { // at least one bit set
            let index = _bitmask.trailing_zeros() as usize; // find that bit
            output[index] = rhs[index].clone(); // copy the value from rhs into output
            _bitmask &= !(1 << index); // set that bit to 0
        }
        return output
    }
}

impl<T> Shl<usize> for Bitmask<T> {
    type Output = Bitmask<T>;

    fn shl(self, rhs: usize) -> Self::Output {
        return Bitmask::<T>::from_u64(self.mask << rhs)
    }
}