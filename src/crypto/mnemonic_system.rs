use crate::crypto::bip39::entropy::generate_entropy_16bit;


pub fn mnemonic_system() {
    let entropy = bip39::entropy::generate_entropy_16bit();
    // let entropy = generate_entropy_16bit();
    // let hashed = sha256_digest_16bit(&entropy);
    // let checksum_result = checksum_16bit(&hashed);

}