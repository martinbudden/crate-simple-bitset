use core::fmt;
use core::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Index};
use serde::{Deserialize, Serialize};

/// A memory-efficient 128-bit set for embedded environments.
#[derive(Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Deserialize, Serialize)]
pub struct BitSet128(u64, u64);

impl BitSet128 {
    /// Create a new empty bitset.
    pub const fn new() -> Self {
        Self(0, 0)
    }
}

impl Default for BitSet128 {
    fn default() -> Self {
        Self::new()
    }
}

impl BitSet128 {
    /// Resets all bits to 0.
    #[inline]
    pub fn reset_all(&mut self) {
        self.0 = 0;
        self.1 = 0;
    }

    /// Resets the bit at `index` to 0. Does nothing if the index is out of bounds.
    #[inline]
    pub fn reset(&mut self, index: u8) {
        if index < 64 {
            self.0 &= !(1u64 << index);
        } else if index < 128 {
            self.1 &= !(1u64 << (index - 64));
        }
    }

    /// Sets all bits to 1.
    #[inline]
    pub fn set_all(&mut self) {
        self.0 = u64::MAX;
        self.1 = u64::MAX;
    }

    /// Sets the bit at `index` to 1. Does nothing if the index is out of bounds.
    #[inline]
    pub fn set(&mut self, index: u8) {
        if index < 64 {
            self.0 |= 1u64 << index;
        } else if index < 128 {
            self.1 |= 1u64 << (index - 64);
        }
    }

    /// Flips the bit at `index`. Does nothing if the index is out of bounds.
    #[inline]
    pub fn flip(&mut self, index: u8) {
        if index < 64 {
            self.0 ^= 1u64 << index;
        } else if index < 128 {
            self.1 ^= 1u64 << (index - 64);
        }
    }

    /// Flips all bits in the bitset (0s become 1s, and 1s become 0s).
    #[inline]
    pub fn flip_all(&mut self) {
        self.0 = !self.0;
        self.1 = !self.1;
    }

    /// In-place Difference / Mask-Clear. Clears any bits that are set in `other`.
    /// This represents the mathematical operation: `self = self AND NOT other`.
    #[inline]
    pub fn and_not(&mut self, other: Self) {
        self.0 &= !other.0;
        self.1 &= !other.1;
    }

    /// Tests if the bit at `index` is 1.
    /// Returns false if the bit is 0 or index is out of bounds.
    #[inline]
    pub fn test(&self, index: u8) -> bool {
        if index < 64 {
            (self.0 & (1u64 << index)) != 0
        } else if index < 128 {
            (self.1 & (1u64 << (index - 64))) != 0
        } else {
            false
        }
    }

    /// Returns the number of set bits (population count).
    #[inline]
    pub const fn count_ones(&self) -> u32 {
        self.0.count_ones() + self.1.count_ones()
    }

    /// Returns the number of leading zeros in the bitset,
    /// counting from the most significant bit (index 127).
    #[inline]
    pub const fn leading_zeros(&self) -> u32 {
        // If the higher 64 bits are completely empty, then all those 64 bits
        // are zeros. We add 64 to whatever leading zeros are found in the lower 64 bits.
        if self.1 == 0 {
            64 + self.0.leading_zeros()
        } else {
            // If the higher 64 bits contain data, the leading zeros of the
            // entire 128-bit structure are determined solely by the higher block.
            self.1.leading_zeros()
        }
    }

    /// Returns true if no bits are set.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0 == 0 && self.1 == 0
    }
    /// Returns the highest index set, or None if the bitset is empty.
    /// Useful for finding the "top" of a priority queue or resource map.
    #[inline]
    pub const fn last_set(&self) -> Option<u8> {
        let leading = self.leading_zeros();
        if leading == 128 {
            None
        } else {
            // Index is 127 minus the number of leading zeros.
            // Example: 0 leading zeros means bit 127 is set.
            // Cast is safe as (127 - leading) is guaranteed to be between 0 and 127.
            #[allow(clippy::cast_possible_truncation)]
            Some((127 - leading) as u8)
        }
    }

    /// Returns true if this bitset contains all the bits set in `other`.
    #[inline]
    pub const fn is_superset(&self, other: &Self) -> bool {
        // A bitset is a superset if clearing any bits not present in self
        // results in exactly the other bitset configuration for both halves.
        (self.0 & other.0) == other.0 && (self.1 & other.1) == other.1
    }

    /// Returns true if this bitset is a subset of `other`.
    #[inline]
    pub const fn is_subset(&self, other: &BitSet128) -> bool {
        other.is_superset(self)
    }

    /// Returns true if this bitset shares at least one common set bit with `other`.
    /// Returns false if there is no overlap or if either bitset is empty.
    #[inline]
    pub const fn intersects(&self, other: &Self) -> bool {
        // Run a bitwise AND between the bitsets.
        // If the result is non-zero, an intersection exists.
        (self.0 & other.0) != 0 || (self.1 & other.1) != 0
    }

    /// Returns an iterator over the indices of the set bits.
    #[inline]
    pub fn iter(&self) -> BitSet128Iter {
        self.into_iter()
    }
}

// **** Bit operations ****

impl BitOr for BitSet128 {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0, self.1 | rhs.1)
    }
}

impl BitAnd for BitSet128 {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0, self.1 & rhs.1)
    }
}

impl BitXor for BitSet128 {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self(self.0 ^ rhs.0, self.1 ^ rhs.1)
    }
}

impl BitOrAssign for BitSet128 {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
        self.1 |= rhs.1;
    }
}

impl BitAndAssign for BitSet128 {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
        self.1 &= rhs.1;
    }
}

impl BitXorAssign for BitSet128 {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
        self.1 ^= rhs.1;
    }
}

impl Index<u8> for BitSet128 {
    type Output = bool;

    fn index(&self, index: u8) -> &Self::Output {
        // We use static booleans because we must return a reference
        if self.test(index) { &true } else { &false }
    }
}

impl Index<usize> for BitSet128 {
    type Output = bool;

    #[allow(clippy::cast_possible_truncation)]
    fn index(&self, index: usize) -> &Self::Output {
        // We use static booleans because we must return a reference
        if self.test(index as u8) { &true } else { &false }
    }
}

/// `BitSet128` from `u32`.
impl From<u32> for BitSet128 {
    #[inline]
    fn from(a: u32) -> Self {
        Self(u64::from(a), 0)
    }
}

/// `BitSet128` from `(u32, u32)`.
impl From<(u32, u32)> for BitSet128 {
    #[inline]
    fn from((a, b): (u32, u32)) -> Self {
        Self(u64::from(a) << 32 | u64::from(b), 0)
    }
}

/// `BitSet128` from `(u32, u32, u32, u32)`.
impl From<(u32, u32, u32, u32)> for BitSet128 {
    #[inline]
    fn from((a, b, c, d): (u32, u32, u32, u32)) -> Self {
        Self(u64::from(a) << 32 | u64::from(b), u64::from(c) << 32 | u64::from(d))
    }
}

// **** Iter ****

#[derive(Debug, Default, Eq, PartialEq)]
pub struct BitSet128Iter(u64, u64);

/// Consuming iterator.
impl Iterator for BitSet128Iter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 == 0 {
            if self.1 == 0 {
                None
            } else {
                // Find the index of the least significant bit set to 1
                #[allow(clippy::cast_possible_truncation)]
                let index = self.1.trailing_zeros() as u8;
                // Clear the least significant bit to prep for next iteration
                self.1 &= self.1 - 1;
                Some(index + 64)
            }
        } else {
            // Find the index of the least significant bit set to 1
            #[allow(clippy::cast_possible_truncation)]
            let index = self.0.trailing_zeros() as u8;
            // Clear the least significant bit to prep for next iteration
            self.0 &= self.0 - 1;
            Some(index)
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        // We know exactly how many bits are left to yield at any moment!
        let len = (self.0.count_ones() + self.1.count_ones()) as usize;
        (len, Some(len))
    }
}

// Implementing ExactSizeIterator unlocks additional optimizations automatically
impl ExactSizeIterator for BitSet128Iter {}
impl core::iter::FusedIterator for BitSet128Iter {}

/// Non-consuming iterator for bitset reference.
impl IntoIterator for &BitSet128 {
    type Item = u8;
    type IntoIter = BitSet128Iter;

    fn into_iter(self) -> Self::IntoIter {
        // Since `BitSet128` is `Copy` and just a `(u64, u64)`,
        // we just pass the underlying value to the iterator.
        BitSet128Iter(self.0, self.1)
    }
}

/// Non-consuming iterator for bitset.
impl IntoIterator for BitSet128 {
    type Item = u8;
    type IntoIter = BitSet128Iter;

    fn into_iter(self) -> Self::IntoIter {
        // Since `BitSet128` is `Copy` and just a `(u64, u64)`,
        // we just pass the underlying value to the iterator.
        BitSet128Iter(self.0, self.1)
    }
}

/// From iterator for bitset.
impl FromIterator<u8> for BitSet128 {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        let mut bitset = Self::new();
        for index in iter {
            bitset.set(index);
        }
        bitset
    }
}

impl Extend<u8> for BitSet128 {
    fn extend<I: IntoIterator<Item = u8>>(&mut self, iter: I) {
        for index in iter {
            self.set(index);
        }
    }
}

// **** fmt ****

impl fmt::Binary for BitSet128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Handle the "0b" prefix if requested via {:#b}
        if f.alternate() {
            f.write_str("0b")?;
        }
        // Print from high bits to low bits (left-to-right)
        // High bits (127 down to 64)
        for i in (0..64).rev() {
            let val = (self.1 >> i) & 1;
            write!(f, "{val}")?;
        }
        // Low bits (63 down to 0)
        for i in (0..64).rev() {
            let val = (self.0 >> i) & 1;
            write!(f, "{val}")?;
        }

        Ok(())
    }
}

impl fmt::UpperHex for BitSet128 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.write_str("0x")?;
        }
        write!(f, "{:016X}{:016X}", self.1, self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_normal<T: Sized + Send + Sync + Unpin>() {}
    fn _is_full<T: Sized + Send + Sync + Unpin + Copy + Clone + Default + PartialEq>() {}
    fn is_config<
        T: Sized + Send + Sync + Unpin + Copy + Clone + Default + PartialEq + Serialize + for<'a> Deserialize<'a>,
    >() {
    }

    #[test]
    fn normal_types() {
        is_config::<BitSet128>();
        is_normal::<BitSet128Iter>();
    }
    #[test]
    fn new() {
        let mut bits = BitSet128::new();
        bits.set(42);
        assert!(bits[42u8]);
        assert!(bits.test(42));
    }
    #[allow(unused)]
    #[test]
    fn const_new() {
        const FLAGS: BitSet128 = BitSet128::new();
        const EMPTY_CHECK: bool = FLAGS.is_empty(); // Evaluated at compile time
    }
    #[test]
    fn assign() {
        let mut bits = BitSet128::new();
        bits.set(42);
        assert!(bits[42u8]);
        assert!(bits.test(42));
        let mask = bits;
        assert!(mask.test(42));
    }
    #[test]
    fn from() {
        let _bits = BitSet128::from((0xab_u32, 0x12_u32));
    }
    #[test]
    fn flip_all() {
        let mut bitset = BitSet128::new();

        // Alternating pattern test
        bitset.set(0);
        bitset.set(64);

        bitset.flip_all();

        // Bit 0 and 64 should now be 0, others should be 1
        assert!(!bitset.test(0));
        assert!(!bitset.test(64));
        assert!(bitset.test(1));
        assert!(bitset.test(65));

        // Full cycle test (Empty -> Full -> Empty)
        let mut empty_set = BitSet128::new();
        empty_set.flip_all(); // Should become full

        let mut full_set = BitSet128::new();
        full_set.set_all();

        assert_eq!(empty_set, full_set);

        empty_set.flip_all(); // Should return to completely empty
        assert!(empty_set.is_empty());
    }
    #[test]
    fn leading_zeros() {
        let mut bitset = BitSet128::new();

        // Completely empty set should return 128 zeros
        assert_eq!(bitset.leading_zeros(), 128);

        // Setting the absolute most significant bit (index 127) leaves 0 leading zeros
        bitset.set(127);
        assert_eq!(bitset.leading_zeros(), 0);

        // Setting index 126 leaves exactly 1 leading zero
        bitset.reset_all();
        bitset.set(126);
        assert_eq!(bitset.leading_zeros(), 1);

        // Test boundary exactly at the edge of the higher u64 (index 64)
        bitset.reset_all();
        bitset.set(64);
        assert_eq!(bitset.leading_zeros(), 63);

        // Test boundary exactly at the edge of the lower u64 (index 63)
        bitset.reset_all();
        bitset.set(63);
        assert_eq!(bitset.leading_zeros(), 64);

        // Setting the absolute lowest bit (index 0) leaves 127 leading zeros
        bitset.reset_all();
        bitset.set(0);
        assert_eq!(bitset.leading_zeros(), 127);
    }
    #[test]
    fn last_set() {
        let mut bitset = BitSet128::new();

        // Empty set should return None
        assert_eq!(bitset.last_set(), None);

        // Single lowest bit set
        bitset.set(0);
        assert_eq!(bitset.last_set(), Some(0));

        // Multiple bits set, should return the highest one
        bitset.set(10);
        bitset.set(45);
        assert_eq!(bitset.last_set(), Some(45));

        // Boundary verification at the top of the lower u64
        bitset.reset_all();
        bitset.set(63);
        assert_eq!(bitset.last_set(), Some(63));

        // Boundary verification at the bottom of the higher u64
        bitset.set(64);
        assert_eq!(bitset.last_set(), Some(64));

        // Multiple bits stretching into the high byte
        bitset.set(100);
        bitset.set(127);
        assert_eq!(bitset.last_set(), Some(127));
    }
    #[test]
    fn is_superset() {
        let mut set_a = BitSet128::new();
        let mut set_b = BitSet128::new();

        // An empty set is always a superset of another empty set
        assert!(set_a.is_superset(&set_b));

        // Setup indices spanning across the 64-bit boundary
        set_a.set(10);
        set_a.set(80);

        set_b.set(10);

        // set_a has [10, 80], set_b has [10] -> Should be true
        assert!(set_a.is_superset(&set_b));
        // set_b is missing 80 -> Should be false
        assert!(!set_b.is_superset(&set_a));

        // Test exact match
        set_b.set(80);
        assert!(set_a.is_superset(&set_b));

        // Test failure where lower matches but higher fails (Catches the original bug)
        let mut set_c = BitSet128::new();
        let mut set_d = BitSet128::new();
        set_c.set(15); // Higher element (.1) is 0

        set_d.set(15);
        set_d.set(95); // Higher element (.1) is non-zero

        // Lower halves match, but set_c is missing bit 95.
        // Your old function would mistakenly return true here.
        assert!(!set_c.is_superset(&set_d));
    }
    #[test]
    fn intersects() {
        let mut set_a = BitSet128::new();
        let mut set_b = BitSet128::new();

        // Empty sets should never intersect
        assert!(!set_a.intersects(&set_b));

        // Add an item to set_a only
        set_a.set(15);
        assert!(!set_a.intersects(&set_b));

        // Match the item in set_b (Intersection in the lower u64 block)
        set_b.set(15);
        assert!(set_a.intersects(&set_b));
        assert!(set_b.intersects(&set_a));

        // Test disjoint sets across the 64-bit split boundary
        set_a.reset_all();
        set_b.reset_all();
        set_a.set(10); // Lower u64 block
        set_b.set(80); // Higher u64 block
        assert!(!set_a.intersects(&set_b));

        // Test intersection purely inside the higher u64 block (index >= 64)
        set_a.set(80);
        assert!(set_a.intersects(&set_b));
    }
    #[test]
    fn inplace_logical_ops() {
        let mut set_a = BitSet128::new();
        let mut set_b = BitSet128::new();

        // Setup overlapping patterns crossing 64-bit boundaries
        set_a.set(10);
        set_a.set(70);

        set_b.set(10);
        set_b.set(80);

        // Test BitAndAssign (&=)
        let mut result = set_a;
        result &= set_b;
        assert!(result.test(10));
        assert!(!result.test(70));
        assert!(!result.test(80));

        // Test BitOrAssign (|=)
        let mut result = set_a;
        result |= set_b;
        assert!(result.test(10));
        assert!(result.test(70));
        assert!(result.test(80));

        // Test BitXorAssign (^=)
        let mut result = set_a;
        result ^= set_b;
        assert!(!result.test(10)); // Both were 1, turns to 0
        assert!(result.test(70));
        assert!(result.test(80));

        // Test and_not
        let mut result = set_a;
        result.and_not(set_b);
        assert!(!result.test(10)); // Cleared by mask
        assert!(result.test(70)); // Preserved
        assert!(!result.test(80)); // Never present in set_a
    }
    #[test]
    fn exercise() {
        let mut system_flags = BitSet128::new();
        let error_mask = BitSet128::new(); // imagine this has error bits set

        // Combine with OR-assign
        system_flags |= error_mask;

        // Toggle bits with XOR-assign
        system_flags ^= error_mask;

        // Mask out bits with AND-assign
        //system_flags &= BitSet128(0x0000_FFFF_FFFF_FFFF);

        let mut set_a = BitSet128::new();
        set_a.set(10);
        set_a.set(20);

        let mut set_b = BitSet128::new();
        set_b.set(20);
        set_b.set(30);

        // Intersection (AND): only bit 20 remains
        let common = set_a & set_b;
        assert!(!common.test(10));
        assert!(common.test(20));
        assert!(!common.test(30));

        // Union (OR): bits 10, 20, and 30 are set
        let all = set_a | set_b;
        assert!(all.test(10));
        assert!(all.test(20));
        assert!(all.test(30));

        // Difference (XOR): bits 10 and 30 are set (20 is cancelled out)
        let diff = set_a ^ set_b;
        assert!(diff.test(10));
        assert!(!diff.test(20));
        assert!(diff.test(30));
    }

    #[test]
    fn test_iterator_empty() {
        let bitset = BitSet128::new();
        let mut iter = bitset.iter();

        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iterator_single_bits() {
        // Test lower bound (index 0)
        let mut bitset = BitSet128::new();
        bitset.set(0);
        let mut iter = bitset.iter();
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), None);

        // Test boundary transition index (63)
        bitset.reset_all();
        bitset.set(63);
        let mut iter = bitset.iter();
        assert_eq!(iter.next(), Some(63));
        assert_eq!(iter.next(), None);

        // Test boundary transition index (64)
        bitset.reset_all();
        bitset.set(64);
        let mut iter = bitset.iter();
        assert_eq!(iter.next(), Some(64));
        assert_eq!(iter.next(), None);

        // Test upper bound (index 127)
        bitset.reset_all();
        bitset.set(127);
        let mut iter = bitset.iter();
        assert_eq!(iter.next(), Some(127));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iterator_multiple_scattered_bits() {
        let mut bitset = BitSet128::new();
        // A stack-allocated expected sequence
        let expected_indices = [5, 12, 63, 64, 99, 127];

        for &idx in &expected_indices {
            bitset.set(idx);
        }

        // Check size_hint tracks perfectly before iteration begins
        let mut iter = bitset.iter();
        assert_eq!(iter.size_hint(), (expected_indices.len(), Some(expected_indices.len())));

        // Verify elements using a zero-allocation loop matching indices
        for &expected in &expected_indices {
            assert_eq!(iter.next(), Some(expected));
        }
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iterator_size_hint_drainage() {
        let mut bitset = BitSet128::new();
        bitset.set(10);
        bitset.set(90);

        let mut iter = bitset.iter();
        assert_eq!(iter.size_hint(), (2, Some(2)));
        assert_eq!(iter.len(), 2); // Enabled by ExactSizeIterator

        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.size_hint(), (1, Some(1)));
        assert_eq!(iter.len(), 1);

        assert_eq!(iter.next(), Some(90));
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.len(), 0);

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iterator_all_bits_set() {
        let mut bitset = BitSet128::new();
        bitset.set_all();

        let mut count = 0;
        #[allow(clippy::cast_possible_truncation)]
        for (expected_idx, actual_idx) in bitset.iter().enumerate() {
            assert_eq!(expected_idx as u8, actual_idx);
            count += 1;
        }
        assert_eq!(count, 128);
    }

    #[test]
    fn test_consuming_into_iter() {
        let mut bitset = BitSet128::new();
        bitset.set(42);

        let mut count = 0;
        for idx in bitset {
            assert_eq!(idx, 42);
            count += 1;
        }
        assert_eq!(count, 1);
    }

    #[test]
    fn from_iterator() {
        // Collect from a concrete array slice
        let indices = [5, 63, 64, 120];
        let bitset: BitSet128 = indices.into_iter().collect();

        assert!(bitset.test(5));
        assert!(bitset.test(63));
        assert!(bitset.test(64));
        assert!(bitset.test(120));
        assert_eq!(bitset.count_ones(), 4);

        // Verify out-of-bounds values are ignored safely without panics
        let invalid_indices = [20, 200, 255];
        let safe_bitset: BitSet128 = invalid_indices.into_iter().collect();

        assert!(safe_bitset.test(20));
        assert_eq!(safe_bitset.count_ones(), 1); // Only index 20 should be set
    }
    #[test]
    fn empty_and_full() {
        let empty = BitSet128::new();
        assert_eq!(empty.iter().count(), 0);

        let mut full = BitSet128::new();
        full.set_all();
        assert_eq!(full.iter().count(), 128);
    }
}
