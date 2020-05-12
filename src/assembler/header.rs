use byteorder::ByteOrder;
use byteorder::LittleEndian;

pub const MAIGC_NUMBER: [u8; 5] = [ 0x6c, 0x75, 0x78, 0x0d, 0x0a ];

pub const HEADER_LENGTH: usize = 64;

pub const VERSION: u16 = 0;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum HeaderErrorKind {
    MagicNumber,
    OutdatedVersion
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodeHeaderError {
    kind: HeaderErrorKind,
}

pub struct Header {
    version: u16,
    pub entry_point: usize,
}

impl Default for Header {
    fn default() -> Self {
        Header {
            version: VERSION,
            entry_point: 0
        }
    }
}

impl Header {
    pub fn decode(bytes: Vec<u8>) -> Result<Header, DecodeHeaderError> {
        let mut i: usize = 0;

        // Verify the magic number
        if bytes[0..MAIGC_NUMBER.len()] != MAIGC_NUMBER {
            return Err(DecodeHeaderError { kind: HeaderErrorKind::MagicNumber })
        }

        i += MAIGC_NUMBER.len();

        let mut header = Header::default();

        header.version = LittleEndian::read_u16(&bytes[i..i + 2]);

        if header.version != VERSION {
            return Err(DecodeHeaderError { kind: HeaderErrorKind::OutdatedVersion })
        }

        i += 2;

        header.entry_point = LittleEndian::read_u32(&bytes[i..i + 4]) as usize;

        Ok(header)
    }

    pub fn bytes(&mut self) -> Vec<u8> {
        let mut header = vec![];
    
        header.append(&mut MAIGC_NUMBER[..].to_vec());

        let mut buf: Vec<u8> = vec![0; 2];
        LittleEndian::write_u16(&mut buf, self.version);
        header.append(&mut buf);

        buf = vec![0; 4];
        LittleEndian::write_u32(&mut buf, self.entry_point as u32);
        header.append(&mut buf);

        println!("{}", buf.len());

        while header.len() < HEADER_LENGTH {
            header.push(0 as u8);
        }
    
        header
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_header() {
        let header = Header::default();

        assert_eq!(header.version, VERSION);
    }

    #[test]
    fn encode_then_decode_header() {
        let mut header = Header::default();

        header.entry_point = 69;

        let bytes: Vec<u8> = header.bytes().to_vec();
        
        assert_eq!(bytes.len(), 64);

        match Header::decode(bytes) {
            Ok(header) => {
                assert_eq!(header.version, VERSION);

                assert_eq!(header.entry_point, 69);
            },
            Err(e) => panic!(e)
        }
    }

    #[test]
    fn fail_on_invalid_header_magic_number() {
        let mut bytes: Vec<u8> = Header::default().bytes().to_vec();

        bytes[0] = 0;

        match Header::decode(bytes) {
            Ok(_) => panic!("Magic number not validated correctly!"),
            Err(_) => { }
        }
    }

    #[test]
    fn fail_on_invalid_header_version() {
        let mut bytes: Vec<u8> = Header::default().bytes().to_vec();

        bytes[MAIGC_NUMBER.len()] += 1;
        bytes[MAIGC_NUMBER.len() + 1] += 1;

        match Header::decode(bytes) {
            Ok(_) => panic!("Version not validated correctly!"),
            Err(_) => { }
        }
    }
}