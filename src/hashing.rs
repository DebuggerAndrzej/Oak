use xxhash_rust::xxh3::xxh3_128;

pub fn get_hashed_input(text: &str) -> u128 {
    return xxh3_128(text.as_bytes()) 
}

