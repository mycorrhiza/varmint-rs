use std::cmp;

/// Returns the number of bytes that would be used to encode the given value as
/// a varint.
pub fn len_u64_varint(val: u64) -> usize {
    let used_bits = u64::min_value().leading_zeros() - val.leading_zeros();
    cmp::max((used_bits + 6) as usize / 7, 1)
}

/// Returns the number of bytes that would be used to encode the given value as
/// a varint.
pub fn len_usize_varint(val: usize) -> usize {
    let used_bits = usize::min_value().leading_zeros() - val.leading_zeros();
    cmp::max((used_bits + 6) as usize / 7, 1)
}

#[cfg(test)]
mod tests {
    use { len_u64_varint, len_usize_varint };

    #[test]
    fn zero() {
        assert_eq!(len_u64_varint(0), 1);
        assert_eq!(len_usize_varint(0), 1);
    }

    #[test]
    fn one() {
        assert_eq!(len_u64_varint(1), 1);
        assert_eq!(len_usize_varint(1), 1);
    }

    #[test]
    fn one_byte_threshold_below() {
        assert_eq!(len_u64_varint(0x7F), 1);
        assert_eq!(len_usize_varint(0x7F), 1);
    }

    #[test]
    fn one_byte_threshold_above() {
        assert_eq!(len_u64_varint(0x80), 2);
        assert_eq!(len_usize_varint(0x80), 2);
    }

    #[test]
    fn some() {
        assert_eq!(len_u64_varint(0x12C), 2);
        assert_eq!(len_usize_varint(0x12C), 2);
    }

    #[test]
    fn many() {
        assert_eq!(len_u64_varint(0x4B3FB5), 4);
        assert_eq!(len_usize_varint(0x4B3FB5), 4);
    }

    #[test]
    fn half() {
        assert_eq!(len_u64_varint(0x7FFFFFFFFFFFFFFF), 9);
    }

    #[test]
    fn all() {
        assert_eq!(len_u64_varint(0xFFFFFFFFFFFFFFFF), 10);
    }
}
