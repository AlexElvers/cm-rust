use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn load_level(path: &Path) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut buffer = vec!();
    file.read_to_end(&mut buffer).unwrap();
    let buffer = &buffer[..];
    let width = u16::from_le_bytes(buffer[..2].try_into().unwrap()) as usize;
    let height = u16::from_le_bytes(buffer[2..4].try_into().unwrap()) as usize;
    if 4 + width * height != buffer.len() {
        panic!(format!("expected exactly {} bytes, found {}", 4 + width * height, buffer.len()));
    }
    buffer[4..].into()
}
