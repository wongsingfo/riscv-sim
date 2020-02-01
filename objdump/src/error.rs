#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    InvalidMagic,
    NotLittleEndianness,
    Not64Bit,
    InvalidStructureSize,
    CanNotFindSection(String),
}

impl std::convert::From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}