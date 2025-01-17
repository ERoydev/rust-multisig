use rand::rngs::OsRng;
use rand::RngCore;

/*
I use this function to create starter random data -> numbers
[154, 241, 123 ... to size]
*/
pub fn generate_entropy_16bit() -> Vec<u8> {
    let mut rng = OsRng {};
    let mut entropy = vec![0u8; 16]; // [0, 0, 0, 0 ...] create empty vector with zeros
    rng.fill_bytes(&mut entropy); // here i fill the vector with random numbers [154, 252, 231 ...]
    entropy // return
}