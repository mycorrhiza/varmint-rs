use std::io;

trait WriteHelper {
    fn write_u8(&mut self, val: u8) -> io::Result<()>;
}

/// A trait to allow writing integers to a byte-oriented sink in the varint
/// format defined by Google's Protocol Buffer standard.
pub trait WriteVarInt {
    /// Write a `u64` to this object encoded as a varint.
    ///
    /// # Errors
    ///
    /// This function will return the first error returned by the underlying
    /// sink.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use varmint::WriteVarInt;
    /// let mut bytes = vec![];
    /// bytes.write_u64_varint(0x4B3FB5).unwrap();
    /// assert_eq!(&bytes[..], &[0xB5, 0xFF, 0xAC, 0x02]);
    /// ```
    fn write_u64_varint(&mut self, val: u64) -> io::Result<()>;

    /// Write a `usize` to this object encoded as a varint.
    ///
    /// # Errors
    ///
    /// This function will return the first error returned by the underlying
    /// sink.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use varmint::WriteVarInt;
    /// let mut bytes = vec![];
    /// bytes.write_usize_varint(0x4B3FB5).unwrap();
    /// assert_eq!(&bytes[..], &[0xB5, 0xFF, 0xAC, 0x02]);
    /// ```
    fn write_usize_varint(&mut self, val: usize) -> io::Result<()>;
}

impl<R: io::Write> WriteHelper for R {
    fn write_u8(&mut self, val: u8) -> io::Result<()> {
        self.write_all(&[val])
    }
}

impl<R: io::Write> WriteVarInt for R {
    fn write_u64_varint(&mut self, mut val: u64) -> io::Result<()> {
        loop {
            let current = (val & 0x7F) as u8;
            val >>= 7;
            if val == 0 {
                try!(self.write_u8(current));
                return Ok(());
            } else {
                try!(self.write_u8(current | 0x80));
            }
        }
    }

    #[cfg(target_arch = "x86_64")] // TODO: better cfg detection of this
    fn write_usize_varint(&mut self, val: usize) -> io::Result<()> {
        self.write_u64_varint(val as u64)
    }
}

#[cfg(test)]
mod tests {
    use WriteVarInt;

    #[test]
    fn zero() {
        let mut bytes = vec![];
        let expected: &[u8] = &[0];
        bytes.write_u64_varint(0).unwrap();
        assert_eq!(&bytes[..], expected);
    }

    #[test]
    fn one() {
        let mut bytes = vec![];
        let expected: &[u8] = &[1];
        bytes.write_u64_varint(1).unwrap();
        assert_eq!(&bytes[..], expected);
    }

    #[test]
    fn some() {
        let mut bytes = vec![];
        let expected: &[u8] = &[0xAC, 0x02];
        bytes.write_u64_varint(0x12C).unwrap();
        assert_eq!(&bytes[..], expected);
    }

    #[test]
    fn many() {
        let mut bytes = vec![];
        let expected: &[u8] = &[0xB5, 0xFF, 0xAC, 0x02];
        bytes.write_u64_varint(0x4B3FB5).unwrap();
        assert_eq!(&bytes[..], expected);
    }

    #[test]
    fn half() {
        let mut bytes = vec![];
        let expected: &[u8] = &[
            0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF,
            0x7F,
        ];
        bytes.write_u64_varint(0x7FFFFFFFFFFFFFFF).unwrap();
        assert_eq!(&bytes[..], expected);
    }

    #[test]
    fn all() {
        let mut bytes = vec![];
        let expected: &[u8] = &[
            0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0x01,
        ];
        bytes.write_u64_varint(0xFFFFFFFFFFFFFFFF).unwrap();
        assert_eq!(&bytes[..], expected);
    }
}
