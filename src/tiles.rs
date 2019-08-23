use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn load_tiles(path: &Path) -> Vec<Vec<u8>> {
    let mut file = File::open(path).unwrap();
    let mut buffer = vec!();
    file.read_to_end(&mut buffer).unwrap();
    let buffer = &buffer[..];
    let mut tiles = vec!();
    let mut i = 0usize;
    while i < buffer.len() {
        let width = u16::from_le_bytes(buffer[i..i + 2].try_into().unwrap()) as usize;
        let height = u16::from_le_bytes(buffer[i + 2..i + 4].try_into().unwrap()) as usize;
        if i + 4 + width * height > buffer.len() {
            panic!("cannot read over array bounds");
        }
        tiles.push(buffer[i + 4..i + 4 + width * height].into());
        i += 4 + width * height;
    }
    tiles
}

#[cfg(test)]
mod test {
    use std::io::Write;

    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn test_load() {
        // create tiles in buffer
        let buffer = {
            let mut buffer = vec![];
            buffer.extend_from_slice(&[20, 0, 20, 0]);
            for i in 0..20 * 20 {
                buffer.push((i % 256) as u8);
            }
            buffer.extend_from_slice(&[10, 0, 15, 0]);
            for i in 0..10 * 15 {
                buffer.push((i % 256) as u8);
            }
            buffer.extend_from_slice(&[2, 0, 2, 0]);
            for i in 0..2 * 2 {
                buffer.push((i % 256) as u8);
            }
            buffer.extend_from_slice(&[20, 0, 20, 0]);
            for i in 0..20 * 20 {
                buffer.push((i % 256) as u8);
            }
            buffer
        };

        let mut file = NamedTempFile::new().unwrap();
        file.write_all(&buffer).unwrap();
        let path = file.into_temp_path();
        let tiles = load_tiles(&path);
        assert_eq!(tiles.len(), 4);
        assert_eq!(tiles[0].len(), 20 * 20);
        assert_eq!(tiles[1].len(), 10 * 15);
        assert_eq!(tiles[2].len(), 2 * 2);
        assert_eq!(tiles[3].len(), 20 * 20);
    }
}
