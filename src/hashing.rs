pub fn murmur_oaat(x: &String) -> u32 {
    let mut hash: u32 = 0x1436781;
    // One-byte-at-a-time hash based on Murmur's mix
    // Source: https://github.com/aappleby/smhasher/blob/master/src/Hashes.cpp
    for &c in x.as_bytes() {
        hash ^= c as u32;
        hash = (std::num::Wrapping(hash) *  std::num::Wrapping(0x5bd1e995)).0;
        hash ^= hash >> 15;
    }

    hash
}