/// Create a DataSize from a number of bits
/// ex: bits!(4)
#[macro_export]
macro_rules! bits {
    ($num_bits:expr) => {
        $crate::datasize::DataSize::new_from_bits($num_bits as u32)
    };
}

/// Create a DataSize from a number of bytes
/// ex: bytes!(4)
#[macro_export]
macro_rules! bytes {
    ($num_bytes:expr) => {
        $crate::datasize::DataSize::new_from_bytes($num_bytes as u32)
    }
}

/// Create a DataSize from a number of kilobytes
/// ex: kilobytes!(4)
#[macro_export]
macro_rules! kilobytes {
    ($num_kilobytes:expr) => {
        $crate::datasize::DataSize::new_from_kilobytes($num_kilobytes as u32)
    }
}

/// Create a DataSize from a number of megabytes
/// ex: megabytes!(4)
#[macro_export]
macro_rules! megabytes {
    ($num_megabytes:expr) => {
        $crate::datasize::DataSize::new_from_megabytes($num_megabytes as u32)
    }
}

/// A generic macro for creating any DataSize type
/// ex: datasize!(4 kilobytes)
#[macro_export]
macro_rules! datasize {
    ($amount:literal $size_type:ident) => {
        match stringify!($size_type) {
            "bits" => bits!($amount),
            "bytes" => bytes!($amount),
            "kilobytes" => kilobytes!($amount),
            "megabytes" => megabytes!($amount),
            val @ _ => panic!("Unsupported size {}", val)
        }
    };
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_macros() {
        assert_eq!(bits!(4), crate::datasize::DataSize::new_from_bits(4));
        assert_eq!(bytes!(4), crate::datasize::DataSize::new_from_bytes(4));
        assert_eq!(kilobytes!(4), crate::datasize::DataSize::new_from_kilobytes(4));
        assert_eq!(megabytes!(4), crate::datasize::DataSize::new_from_megabytes(4));
        assert_eq!(datasize!(2 bits), bits!(2));
        assert_eq!(datasize!(2 bytes), bytes!(2));
        assert_eq!(datasize!(2 kilobytes), kilobytes!(2));
        assert_eq!(datasize!(2 megabytes), megabytes!(2));
    }

    #[test]
    #[should_panic]
    fn test_bad_size_type() {
        let _ = datasize!(2 petabytes);
    }
}
