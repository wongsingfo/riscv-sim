pub(crate) fn array_as_u16(array: &[u8]) -> u16 {
    array[0] as u16 +
        ((array[1] as u16) << 8)
}

pub(crate) fn array_as_u32(array: &[u8]) -> u32 {
    array_as_u16(&array[0..2]) as u32 +
        ((array_as_u16(&array[2..4]) as u32) << 16)
}

pub(crate) fn array_as_u64(array: &[u8]) -> u64 {
    array_as_u32(&array[0..4]) as u64 +
        ((array_as_u32(&array[4..8]) as u64) << 32)
}
#[test]
fn test_array_as_u64() {
    let a: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    assert_eq!(array_as_u16(&a), 0x0201);
    assert_eq!(array_as_u64(&a), 0x0807060504030201);
}