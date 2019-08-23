use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn load_palette(path: &Path) -> Vec<Vec<u8>> {
    let mut file = File::open(path).unwrap();
    let mut buffer = vec!();
    file.read_to_end(&mut buffer).unwrap();
    if buffer.len() % 3 != 0 {
        panic!("palette file is corrupted: invalid length");
    }
    buffer.iter().map(|v| 4 * v).collect::<Vec<u8>>()
        .chunks_exact(3).enumerate().map(
        |(i, v)| {
            [v, &[if i == 0 { 0 } else { 255 }]].concat()
        }
    ).collect()
}
