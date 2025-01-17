use sha256::digest;

/*
Used to turn my entropy data into hash
*/
pub fn sha256_digest_16bit(data: &Vec<u8>) -> String {
    let hash = digest(data);
    hash
}