
pub fn checksum_16bit(hash: &String) -> String {
    let first_two = &hash[..2];
    let binary = format!("{:08b}", u8::from_str_radix(first_two, 16).unwrap());
    binary
}