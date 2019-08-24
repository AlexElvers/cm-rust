use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn load_palette(path: &Path) -> Result<Vec<Vec<u8>>, String> {
    let mut file = File::open(path).unwrap();
    let mut buffer = vec!();
    file.read_to_end(&mut buffer).unwrap();
    if buffer.len() % 3 != 0 {
        return Err(format!(
            "palette file {} is corrupted: length has to be divisible by 3, was {}",
            path.display(), buffer.len(),
        ));
    }
    Ok(buffer.iter().map(|v| 4 * v).collect::<Vec<u8>>()
        .chunks_exact(3).enumerate().map(
        |(i, v)| {
            [v, &[if i == 0 { 0 } else { 255 }]].concat()
        }
    ).collect())
}
