use crate::datasize::DataSize;

pub trait Fits {
    fn fits_in(&self, size: &DataSize) -> bool;
}

/// Whether or not the value of a u32 can fit into a given [DataSize]
impl Fits for u32 {
    fn fits_in(&self, size: &DataSize) -> bool {
        match self {
            _f if size.max_value() >= *self => true,
            _ => false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fits() {
        assert_eq!(3u32.fits_in(&datasize!(3 bits)), true);
        assert_eq!(3u32.fits_in(&datasize!(1 bits)), false);
    }
}
