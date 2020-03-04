use std::fmt::{Display, Debug, Formatter, Result};
use std::ops::Add;

/**
 * An amount of data, modeled as a number of a bits.
 * Can represent a maximum of u32::max_limit() bits.
 */
#[derive(PartialEq, PartialOrd, Debug)]
pub struct DataSize {
    // Using u32 here is an arbitrary limit
    num_bits: u32
}

impl DataSize {
    /**
     * Create a DataSize from a number of bits
     */
    pub fn new_from_bits(num_bits: u32) -> DataSize {
        DataSize { num_bits }
    }

    /**
     * Create a DataSize from a number of bytes
     */
    pub fn new_from_bytes(num_bytes: u32) -> DataSize {
        match num_bytes.checked_mul(DataSize::BYTE_IN_BITS) {
            Some(num_bits) => DataSize { num_bits },
            _ => panic!("Unsupported number of bytes: {}, it cannot fit in a u32 as a number of bits", num_bytes)
        }
    }

    /**
     * Create a DataSize from a number of kilobytes
     */
    pub fn new_from_kilobytes(num_kilobytes: u32) -> DataSize {
        match num_kilobytes.checked_mul(DataSize::KILOBYTE_IN_BITS) {
            Some(num_bits) => DataSize { num_bits },
            _ => panic!("Unsupported number of kilobytes: {}, it cannot fit in a u32 as a number of bits", num_kilobytes)
        }
    }

    /**
     * Create a DataSize from a number of megabytes
     */
    pub fn new_from_megabytes(num_megabytes: u32) -> DataSize {
        match num_megabytes.checked_mul(DataSize::MEGABYTE_IN_BITS) {
            Some(num_bits) => DataSize { num_bits },
            _ => panic!("Unsupported number of megabytes: {}, it cannot fit in a u32 as a number of bits", num_megabytes)
        }
    }

    /**
     * Return the number of bits represented by this DataSize
     */
    pub fn bits(&self) -> u32 { self.num_bits }
    /**
     * Return the (truncated) number of bytes represented by this DataSize
     */
    pub fn bytes(&self) -> u32 { self.num_bits / 8 }
    /**
     * Return the (truncated) number of kilobytes represented by this DataSize
     */
    pub fn kilobytes(&self) -> u32 { self.bytes() / 1000 }
    /**
     * Return the (truncated) number of megabytes represented by this DataSize
     */
    pub fn megabytes(&self) -> u32 { self.kilobytes() / 1000 }

    const BYTE_IN_BITS: u32 = 8;
    const KILOBYTE_IN_BITS: u32 = 1000 * DataSize::BYTE_IN_BITS;
    const MEGABYTE_IN_BITS: u32 = 1000 * DataSize::KILOBYTE_IN_BITS;
}

impl Add for DataSize {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            num_bits: self.num_bits + other.num_bits
        }
    }
}

impl Display for DataSize {
    fn fmt(&self, f: &mut Formatter) -> Result {
        // Not sure if there's a good 'match' statement that could be
        // used here, since I'm trying to avoid calling every function
        // when it's likely not all will be needed (we can stop at
        // the first one that's non-zero).
        let (value, descriptor)  = if self.megabytes() == 1 {
            (self.megabytes(), "megabyte")
        } else if self.megabytes() > 1 {
            (self.megabytes(), "megabytes")
        } else if self.kilobytes() == 1 {
            (self.kilobytes(), "kilobyte")
        } else if self.kilobytes() > 1 {
            (self.kilobytes(), "kilobytes")
        } else if self.bytes() == 1 {
            (self.bytes(), "byte")
        } else if self.bytes() > 1 {
            (self.bytes(), "bytes")
        } else if self.bits() == 1 {
            (self.bits(), "bit")
        } else {
            (self.bits(), "bits")
        };

        write!(f, "{} {}", value, descriptor)
    }
}

// Helper macros

#[macro_export]
macro_rules! bits {
    ($num_bits:literal) => {
        DataSize::new_from_bits($num_bits as u32)
    }
}

#[macro_export]
macro_rules! bytes {
    ($num_bytes:literal) => {
        DataSize::new_from_bytes($num_bytes as u32)
    }
}

#[macro_export]
macro_rules! kilobytes {
    ($num_kilobytes:literal) => {
        DataSize::new_from_kilobytes($num_kilobytes as u32)
    }
}

#[macro_export]
macro_rules! megabytes {
    ($num_megabytes:literal) => {
        DataSize::new_from_megabytes($num_megabytes as u32)
    }
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equality() {
        assert!(DataSize::new_from_bits(8) == DataSize::new_from_bytes(1));
    }

    #[test]
    fn test_ordering() {
        assert!(DataSize::new_from_bits(1) < DataSize::new_from_bytes(1));
        assert!(DataSize::new_from_bits(10) > DataSize::new_from_bytes(1));
    }

    #[test]
    #[should_panic]
    fn test_megabytes_overflow() {
        DataSize::new_from_megabytes(u32::max_value());
    }

    #[test]
    fn test_macros() {
        assert_eq!(bits!(4), DataSize::new_from_bits(4));
        assert_eq!(bytes!(4), DataSize::new_from_bytes(4));
        assert_eq!(kilobytes!(4), DataSize::new_from_kilobytes(4));
        assert_eq!(megabytes!(4), DataSize::new_from_megabytes(4));
    }

    #[test]
    fn test_addition() {
        assert_eq!(bits!(4) + bits!(4), bytes!(1));
        assert_eq!(bytes!(2) + bits!(4), bits!(20));
    }
}
