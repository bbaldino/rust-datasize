use crate::datasize::DataSize;

/// A generic macro for creating any DataSize type
/// ex: datasize!(4 kilobytes)
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

/// Create a DataSize from a number of bits
/// ex: bits!(4)
#[macro_export]
macro_rules! bits {
    ($num_bits:expr) => {
        DataSize::new_from_bits($num_bits as u32)
    };
}

/// Create a DataSize from a number of bytes
/// ex: bytes!(4)
#[macro_export]
macro_rules! bytes {
    ($num_bytes:expr) => {
        DataSize::new_from_bytes($num_bytes as u32)
    }
}

/// Create a DataSize from a number of kilobytes
/// ex: kilobytes!(4)
#[macro_export]
macro_rules! kilobytes {
    ($num_kilobytes:expr) => {
        DataSize::new_from_kilobytes($num_kilobytes as u32)
    }
}

/// Create a DataSize from a number of megabytes
/// ex: megabytes!(4)
#[macro_export]
macro_rules! megabytes {
    ($num_megabytes:expr) => {
        DataSize::new_from_megabytes($num_megabytes as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
