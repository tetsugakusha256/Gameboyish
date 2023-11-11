pub fn u8_array_to_string(arr: &[u8]) -> String {
    // Use the iterator over the array to convert each byte to a hexadecimal string
    let hex_string: String = arr.iter().map(|&byte| format!("{:02X} ", byte)).collect();

    // Return the concatenated hexadecimal string
    hex_string.trim().to_string()
}
pub fn u8_array_to_individual_char(arr: &[u8]) -> String {
    // Use the iterator over the array to convert each byte to a hexadecimal string
    let hex_string: String = arr.iter().map(|&byte| format!("{}", char::from(byte))).collect();

    // Return the concatenated hexadecimal string
    hex_string.trim().to_string()
}
#[cfg(test)]
mod tests {
    use crate::util::display_impl::u8_array_to_string;
    #[test]
    fn u8_array_to_string_test() {
        let text = u8_array_to_string(&[0xE8,8u8,0b11,0xeC]);
        println!("text: {}",text);
        assert_eq!(u8_array_to_string(&[5u8,8u8]), "05 08");
        assert_eq!(u8_array_to_string(&[0xFF,8u8]), "FF 08");
        assert_eq!(u8_array_to_string(&[0xFC,0xA5]), "FC A5");
        assert_eq!(u8_array_to_string(&[0xFF,9u8]), "FF 09");
        assert_eq!(u8_array_to_string(&[0xff,9u8]), "FF 09");
    }
}
