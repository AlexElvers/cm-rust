use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::objects::Object;

#[derive(Copy, Clone, Debug)]
pub enum Direction { North, East, South, West }

#[derive(Copy, Clone, Debug)]
pub struct Cell {
    pub x: u16,
    pub y: u16,
    pub object: Option<Object>,
    pub pre_occupied: bool,
    pub post_occupied: bool,
    pub changed_in_current_tick: bool,
    pub moving_in_from: Option<Direction>,
}

impl Cell {
    pub fn is_transparent(self) -> bool {
        match self.object {
            Some(object) => object.is_transparent(),
            None => true,
        }
    }

    pub fn can_be_entered(self) -> bool {
        match self.object {
            Some(object) => object.can_be_entered(),
            None => true,
        }
    }
}

/// A level contains a map, i.e., a collection of row-by-row tile indices.  The
/// width and height are the number of tiles per row respectively column of the
/// map; therefore, the map should have width*height entries.
pub struct Level {
    pub width: u16,
    pub height: u16,
    pub map: Vec<Cell>,
}

impl Level {
    pub fn cell(&self, x: u16, y: u16) -> Option<&Cell> {
        self.map.get((y * self.width + x) as usize)
    }

    pub fn cell_checked(&self, x: u16, y: u16) -> Option<&Cell> {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.map.get((y * self.width + x) as usize)
        } else {
            None
        }
    }

    pub fn set_cell(&mut self, x: u16, y: u16, cell: Cell) {
        if let Some(cell_ref) = self.map.get_mut((y * self.width + x) as usize) {
            *cell_ref = cell;
        }
    }
}


/// A level file is composed of a header (width and height) and the map data
/// (width*height) entries.
pub fn load_level(path: &Path) -> Result<Level, String> {
    let mut file = File::open(path).unwrap();
    let mut buffer = vec!();
    file.read_to_end(&mut buffer).unwrap();
    if buffer.len() < 4 {
        return Err(format!("missing header (width and height) in level file {}", path.display()));
    }
    let width = u16::from_le_bytes(buffer[..2].try_into().unwrap());
    let height = u16::from_le_bytes(buffer[2..4].try_into().unwrap());
    if 4 + width as usize * height as usize == buffer.len() {
        Ok(Level {
            width,
            height,
            map: buffer[4..].iter().enumerate().map(
                |(i, &x)| Cell {
                    x: i as u16 % width,
                    y: i as u16 / width,
                    object: match Object::from_tile_number(x).unwrap() {
                        Object::Empty => None,
                        object => Some(object),
                    },
                    pre_occupied: false,
                    post_occupied: false,
                    changed_in_current_tick: false,
                    moving_in_from: None,
                }
            ).collect(),
        })
    } else {
        Err(format!(
            "according to header, level file {} should contain 4+{}*{}={} bytes, found {} bytes",
            path.display(), width, height, 4 + width * height, buffer.len(),
        ))
    }
}


#[cfg(test)]
mod test {
    use std::io::Write;

    use tempfile::{NamedTempFile, TempPath};

    use super::*;

    fn create_temp_file(data: &[u8]) -> TempPath {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(data).unwrap();
        file.into_temp_path()
    }

    #[test]
    fn test_load() {
        // missing header, requires at least 4 bytes
        let path = create_temp_file(&[9]);
        assert_eq!(load_level(&path).err(), Some(format!(
            "missing header (width and height) in level file {}", path.display(),
        )));

        // map data too short
        let path = create_temp_file(&[9, 0, 2, 0, 99, 99]);
        assert_eq!(load_level(&path).err(), Some(format!(
            "according to header, level file {} should contain 4+9*2=22 bytes, found 6 bytes",
            path.display(),
        )));

        // map data too long
        let path = create_temp_file(&[1, 0, 1, 0, 99, 99]);
        assert_eq!(load_level(&path).err(), Some(format!(
            "according to header, level file {} should contain 4+1*1=5 bytes, found 6 bytes",
            path.display(),
        )));

        // success
        let path = create_temp_file(&[
            4, 0, 3, 0,
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
        ]);
        let level = load_level(&path).unwrap();
        assert_eq!(level.width, 4);
        assert_eq!(level.height, 3);
        assert_eq!(level.map, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    }
}
