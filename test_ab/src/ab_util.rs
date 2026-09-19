//! A/B Test Util

// Load PEM into Vec
// @std @alloc
pub fn load_pem_vec(path: &str) -> Vec<u8> {
    use std::io::Read;
    let mut f = std::fs::File::open(path).unwrap();
    let mut data: Vec<u8> = vec![];
    f.read_to_end(&mut data).unwrap();
    data
}
