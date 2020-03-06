use std::fmt::{Display, Debug, Formatter, Result};
use std::ops::{Add, Sub};

/**
 * An amount of data, modeled as a number of a bits and readable
 * as an amount of bits, bytes, kilobytes or megabytes.
 *
 * Can represent a maximum of u32::max_limit() bits.
 */
#[derive(PartialEq, PartialOrd, Debug, Clone)]
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

    /// Return the max value this [DataSize] could hold
    pub fn max_value(&self) -> u32 {
        let mut max_value = 0u32;
        for _ in 0..self.bits() - 1 {
            max_value = max_value | 1;
            max_value = max_value << 1;
        }
        // Do the last 'or' here so we don't shift again
        max_value | 1
    }

    const BYTE_IN_BITS: u32 = 8;
    const KILOBYTE_IN_BITS: u32 = 1000 * DataSize::BYTE_IN_BITS;
    const MEGABYTE_IN_BITS: u32 = 1000 * DataSize::KILOBYTE_IN_BITS;
}

impl Add for DataSize {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        match self.num_bits.checked_add(other.num_bits) {
            Some(new_total) => DataSize { num_bits: new_total },
            _ => panic!("Addition results in an overflow: {} + {} bits can't fit in a u32", self.num_bits, other.num_bits)
        }
    }
}

impl Sub for DataSize {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match self.num_bits.checked_sub(other.num_bits) {
            Some(num_bits) => DataSize { num_bits },
            _ => panic!("Subtraction results in a negative number: {} - {}", self.num_bits, other.num_bits),
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

/**
 * A generic macro for creating any DataSize type
 * ex: datasize!(4 kilobytes)
 */
#[macro_export]
macro_rules! datasize {
    ($num_bits:literal bits) => {
        DataSize::new_from_bits($num_bits as u32)
    };
    ($num_bytes:literal bytes) => {
        DataSize::new_from_bytes($num_bytes as u32)
    };
    ($num_kilobytes:literal kilobytes) => {
        DataSize::new_from_kilobytes($num_kilobytes as u32)
    };
    ($num_megabytes:literal megabytes) => {
        DataSize::new_from_megabytes($num_megabytes as u32)
    };
}

/**
 * Create a DataSize from a number of bits
 * ex: bits!(4)
 */
#[macro_export]
macro_rules! bits {
    ($num_bits:expr) => {
        DataSize::new_from_bits($num_bits as u32)
    };
}

/**
 * Create a DataSize from a number of bytes
 * ex: bytes!(4)
 */
#[macro_export]
macro_rules! bytes {
    ($num_bytes:expr) => {
        DataSize::new_from_bytes($num_bytes as u32)
    }
}

/**
 * Create a DataSize from a number of kilobytes
 * ex: kilobytes!(4)
 */
#[macro_export]
macro_rules! kilobytes {
    ($num_kilobytes:expr) => {
        DataSize::new_from_kilobytes($num_kilobytes as u32)
    }
}

/**
 * Create a DataSize from a number of megabytes
 * ex: megabytes!(4)
 */
#[macro_export]
macro_rules! megabytes {
    ($num_megabytes:expr) => {
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
        assert_eq!(datasize!(2 bits), bits!(2));
        assert_eq!(datasize!(2 bytes), bytes!(2));
        assert_eq!(datasize!(2 kilobytes), kilobytes!(2));
        assert_eq!(datasize!(2 megabytes), megabytes!(2));
    }

    #[test]
    fn test_addition() {
        assert_eq!(bits!(4) + bits!(4), bytes!(1));
        assert_eq!(bytes!(2) + bits!(4), bits!(20));
    }

    #[test]
    #[should_panic]
    fn test_addition_overflow() {
        #[allow(unused_must_use)] {
            bits!(u32::max_value()) + bits!(1);
        }
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(bits!(4) - bits!(2), bits!(2));
        assert_eq!(bits!(4) - bits!(4), bytes!(0));
    }

    #[test]
    #[should_panic]
    fn test_subtraction_overflow() {
        #[allow(unused_must_use)] {
            bits!(1) - bits!(3);
        }
    }

    #[test]
    fn test_max_value() {
        assert_eq!(bits!(2).max_value(), 3);
        assert_eq!(bytes!(2).max_value(), 65535);
    }
}
