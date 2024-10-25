#[cfg(test)]
mod tests {
    #[test]
    fn u16_to_u32() {
        // 将 47u16 转换为 u32
        let v: u32 = 47u16 as u32;
        assert_eq!(47u16 as u32, v);
    }

    #[test]
    fn u8_to_i8() {
        // 禁用溢出字面值检查，并将 255 转换为 i8，这会导致溢出，并取值为 -1
        #[allow(overflowing_literals)]
            let x = { 255 as i8 };

        // 255u8 在转换为 i8 时，相当于 -1，因为它超过了 i8 的范围
        let y: i8 = -1;

        assert_eq!(x, y);
    }

    #[test]
    fn bool_to_u8() {
        // 将 true 转换为 u8，true 相当于 1
        let v: u8 = true as u8;
        assert_eq!(true as u8, v);
    }
}
