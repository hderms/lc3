pub fn sign_extend(x: u16, bit_count: usize) -> u16 {
    if ((x >> (bit_count - 1)) & 1) == 1 {
        x | (0xFFFF << bit_count)
    } else {
        x
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_sign_extends() {
        assert_eq!(sign_extend(0b111111, 5), 0xFFFF);
        assert_eq!(sign_extend(0b11110, 5), 0xFFFE);
        assert_eq!(sign_extend(0b1110, 4), 0xFFFE);

        assert_eq!(sign_extend(0b1110, 4) as i16, -2);
        assert_eq!(sign_extend(1, 8) as i16, 1);
        assert_eq!(sign_extend(1, 9) as i16, 1);
    }
}
