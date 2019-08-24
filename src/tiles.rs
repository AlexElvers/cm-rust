use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Tile {
    pub width: u16,
    pub height: u16,
    pub data: Vec<u8>,
}


pub fn load_tiles(path: &Path) -> Result<Vec<Tile>, String> {
    let mut file = File::open(path).unwrap();
    let mut buffer = vec!();
    file.read_to_end(&mut buffer).unwrap();
    let buffer = &buffer[..];
    let mut tiles = vec!();
    let mut i = 0;
    while i < buffer.len() {
        let width = u16::from_le_bytes(buffer[i..i + 2].try_into().unwrap());
        let height = u16::from_le_bytes(buffer[i + 2..i + 4].try_into().unwrap());
        if i + 4 + width as usize * height as usize > buffer.len() {
            return Err("cannot read over array bounds".into());
        }
        tiles.push(Tile {
            width,
            height,
            data: buffer[i + 4..i + 4 + width as usize * height as usize].into(),
        });
        i += 4 + width as usize * height as usize;
    }
    Ok(tiles)
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
        let tiles = load_tiles(&path).unwrap();
        assert_eq!(tiles.len(), 4);
        assert_eq!(tiles[0].width, 20);
        assert_eq!(tiles[0].height, 20);
        assert_eq!(tiles[0].data.len(), 20 * 20);
        assert_eq!(tiles[1].width, 10);
        assert_eq!(tiles[1].height, 15);
        assert_eq!(tiles[1].data.len(), 10 * 15);
        assert_eq!(tiles[2].width, 2);
        assert_eq!(tiles[2].height, 2);
        assert_eq!(tiles[2].data.len(), 2 * 2);
        assert_eq!(tiles[3].width, 20);
        assert_eq!(tiles[3].height, 20);
        assert_eq!(tiles[3].data.len(), 20 * 20);
    }
}
