use std::io;

trait ReadHelper {
    fn read_u8(&mut self) -> io::Result<u8>;
    fn try_read_u8(&mut self) -> io::Result<Option<u8>>;
    fn read_remaining_u64_varint(&mut self, first: u8) -> io::Result<u64>;
}

pub trait ReadVarInt {
    fn read_u64_varint(&mut self) -> io::Result<u64>;
    fn read_usize_varint(&mut self) -> io::Result<usize>;

    /// Returns None if EOF on the first byte
    fn try_read_u64_varint(&mut self) -> io::Result<Option<u64>>;

    /// Returns None if EOF on the first byte
    fn try_read_usize_varint(&mut self) -> io::Result<Option<usize>>;
}

impl<R: io::Read> ReadHelper for R {
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut buffer = [0];
        try!(self.read_exact(&mut buffer));
        Ok(buffer[0])
    }

    fn try_read_u8(&mut self) -> io::Result<Option<u8>> {
        let mut buffer = [0];
        if try!(self.read(&mut buffer)) == 1 {
            Ok(Some(buffer[0]))
        } else {
            Ok(None)
        }
    }

    fn read_remaining_u64_varint(&mut self, first: u8) -> io::Result<u64> {
        if first & 0x80 == 0 {
            return Ok(first as u64);
        }

        let mut result = (first & 0x7F) as u64;
        let mut offset = 7;

        loop {
            let current = try!(self.read_u8());
            result = result + (((current & 0x7F) as u64) << offset);
            if current & 0x80 == 0 {
                return Ok(result);
            }
            offset += 7;
            if offset == 63 {
                let last = try!(self.read_u8());
                if last == 0x01 {
                    return Ok(result + (1 << offset));
                } else {
                    return Err(io::Error::new(
                            io::ErrorKind::Other,
                            "varint exceeded 64 bits long"));
                }
            }
        }
    }
}

impl<R: io::Read> ReadVarInt for R {
    fn read_u64_varint(&mut self) -> io::Result<u64> {
        let first = try!(self.read_u8());
        self.read_remaining_u64_varint(first)
    }

    fn try_read_u64_varint(&mut self) -> io::Result<Option<u64>> {
        if let Some(first) = try!(self.try_read_u8()) {
            Ok(Some(try!(self.read_remaining_u64_varint(first))))
        } else {
            Ok(None)
        }
    }

    #[cfg(target_arch = "x86_64")] // TODO: better cfg detection of this
    fn read_usize_varint(&mut self) -> io::Result<usize> {
        self.read_u64_varint().map(|u| u as usize)
    }

    #[cfg(target_arch = "x86_64")] // TODO: better cfg detection of this
    fn try_read_usize_varint(&mut self) -> io::Result<Option<usize>> {
        self.try_read_u64_varint().map(|o| o.map(|u| u as usize))
    }
}

#[cfg(test)]
mod tests {
    use { ReadVarInt };

    #[test]
    fn zero() {
        let mut bytes: &[u8] = &[0];
        assert_eq!(bytes.read_u64_varint().unwrap(), 0);
    }

    #[test]
    fn one() {
        let mut bytes: &[u8] = &[1];
        assert_eq!(bytes.read_u64_varint().unwrap(), 1);
    }

    #[test]
    fn some() {
        let mut bytes: &[u8] = &[0xAC, 0x02];
        assert_eq!(bytes.read_u64_varint().unwrap(), 0x12C);
    }

    #[test]
    fn many() {
        let mut bytes: &[u8] = &[0xB5, 0xFF, 0xAC, 0x02];
        assert_eq!(bytes.read_u64_varint().unwrap(), 0x4B3FB5);
    }

    #[test]
    fn half() {
        let mut bytes: &[u8] = &[
            0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF,
            0x7F,
        ];
        assert_eq!(bytes.read_u64_varint().unwrap(), 0x7FFFFFFFFFFFFFFF);
    }

    #[test]
    fn all() {
        let mut bytes: &[u8] = &[
            0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0x01,
        ];
        assert_eq!(bytes.read_u64_varint().unwrap(), 0xFFFFFFFFFFFFFFFF);
    }

    #[test]
    fn too_many() {
        let mut bytes: &[u8] = &[
            0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0xFF, 0xFF, 0xFF,
            0xFF, 0x02,
        ];
        assert!(bytes.read_u64_varint().is_err());
    }

    #[test]
    fn try_some() {
        let mut bytes: &[u8] = &[0xAC, 0x02];
        assert_eq!(bytes.try_read_u64_varint().unwrap(), Some(0x12C));
    }

    #[test]
    fn try_none() {
        let mut bytes: &[u8] = &[];
        assert_eq!(bytes.try_read_u64_varint().unwrap(), None);
    }
}
