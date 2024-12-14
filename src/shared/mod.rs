/// Evaluates if a bitmask has only one bit set
/// 
/// # examples
/// 
/// ```
/// // a bitflag that only has the lowest bit set
/// assert_eq!(has_one_bit_set(1u64), true)
/// 
/// // a bitflag with two bits set
/// assert_eq!(has_one_bit_set(2u64 | 4u64), false)
/// 
/// // a bitflag with no bits set
/// assert_eq!(has_one_bit_set(0u64), false)
/// ```
pub(crate) fn has_one_bit_set(bitmask: u64) -> bool {
    bitmask != 0 && (bitmask & (bitmask - 1)) == 0
}

/// Takes a bitflag representing a single position
/// and returns the u8 key from enumerating positions.
/// Note that this has no validation: if you pass in
/// a bitflag with multiple set bits, it will return
/// the key for the lowest set index, which can cause
/// unexpected behaviour.
/// 
/// # Examples
///
/// ```
/// // A bitflag with only the lowest bit set (1 << 0)
/// assert_eq!(single_bit_bitmask_to_u8(0b00000001), 0);
///
/// // A bitflag with the 4th bit set (1 << 3)
/// assert_eq!(single_bit_bitmask_to_u8(0b00001000), 3);
///
/// // A bitflag with the 7th bit set (1 << 6)
/// assert_eq!(single_bit_bitmask_to_u8(0b01000000), 6);
///
/// // Multiple bits set: returns the lowest index (0)
/// assert_eq!(single_bit_bitmask_to_u8(0b00001001), 0);
/// ```
pub(crate) fn single_bit_bitmask_to_u8(bitmask: &u64) -> u8 {
    bitmask.trailing_zeros() as u8
}

/// Takes a bitflag and returns a `Vec<u8>` containing
/// the positions of all the set bits, from lowest to highest.
/// 
/// # Examples
///
/// ```
/// // A bitmask with bits 0, 3, and 6 set
/// assert_eq!(multi_bitmask_to_u8s(0b01001001), vec![0, 3, 6]);
///
/// // A bitmask with only one bit set
/// assert_eq!(multi_bitmask_to_u8s(0b00000010), vec![1]);
///
/// // A bitmask with all bits set (in an 8-bit context)
/// assert_eq!(multi_bitmask_to_u8s(0b11111111), vec![0, 1, 2, 3, 4, 5, 6, 7]);
///
/// // An empty bitmask (no bits set)
/// assert_eq!(multi_bitmask_to_u8s(0b00000000), vec![]);
/// ```
pub(crate) fn multi_bitmask_to_u8s(bitmask: &u64) -> Vec<u8> {
    // take a copy for deconstruction
    let mut bitmask = *bitmask;
    let mut output: Vec<u8> = Vec::new();
    while bitmask != 0 {
        output.push(bitmask.trailing_zeros() as u8);
        bitmask &= !(1 << output.last().expect("Contains at least one value"))
    }
    output
}

