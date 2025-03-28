pub fn is_address_valid(address: &String) -> bool {
    // An address is always 40 hexadecimal characters (or 2 more character with 0x prefix)
    if address.len() != 40 && address.len() != 42 {
        return false;
    }

    true
}



// pub fn bytes_to_eth(bytes: &[u8; 20]) -> String {
//     // 转换为十六进制字符串
//     let hex_str = hex::encode(bytes);
//     // 添加 0x 前缀
//     format!("{}", hex_str)
// }
