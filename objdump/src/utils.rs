use byteorder::{ByteOrder, LittleEndian};

pub(crate) fn array_as_u16(array: &[u8]) -> u16 {
    LittleEndian::read_u16(array)
}

pub(crate) fn array_as_u32(array: &[u8]) -> u32 {
    LittleEndian::read_u32(array)
}

pub(crate) fn array_as_u64(array: &[u8]) -> u64 {
    LittleEndian::read_u64(array)
}

#[test]
fn test_array_as_u64() {
    let a: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(array_as_u16(&a), 0x0201);
    assert_eq!(array_as_u64(&a), 0x0807060504030201);
}