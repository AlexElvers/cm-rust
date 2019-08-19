use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
pub struct HighScore {
    name: String,
    score: u32,
}


/**
 * High score table files contain multiple high score entries. The format of an
 * entry looks as follows:
 * - player name (3 bytes, encoded as 0=A, 1=B etc.)
 * - padding (1 byte, should be 0)
 * - score (4 bytes, little endian)
 */
fn parse(buffer: &[u8]) -> Result<Vec<HighScore>, String> {
    let mut high_scores = vec!();
    let mut iter = buffer.chunks_exact(8);
    for chunk in iter.by_ref() {
        let name_buffer = &chunk[..3];
        if !name_buffer.iter().all(|c| *c < 26) {
            return Err(String::from("name contains invalid characters"));
        }
        let name = match String::from_utf8(name_buffer.iter().map(|c| c + 65).collect()) {
            Ok(name) => name,
            Err(_) => return Err(String::from("cannot parse name")),
        };
        let score = chunk[4..].iter().enumerate().map(|(i, &c)| (c as u32) << (i * 8) as u32).sum::<u32>();
        if score == 0 {
            break;
        }
        high_scores.push(HighScore { name, score });
    }
    let remainder = iter.remainder();
    if remainder.len() > 0 {
        return Err(String::from("unparsed bytes at end"));
    }
    Ok(high_scores)
}


pub fn load(path: &Path) -> Result<Vec<HighScore>, String> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(reason) => return Err(format!("cannot open {}: {:?}", path.display(), reason)),
    };
    let mut buffer = vec!();
    if let Err(reason) = file.read_to_end(&mut buffer) {
        return Err(format!("cannot read {}: {:?}", path.display(), reason));
    }
    parse(&buffer)
}


#[cfg(test)]
mod test {
    use std::io::Write;

    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn test_parse() {
        let buffer = [
            9, 0, 12, 0, 89, 4, 0, 0,
            0, 1, 2, 0, 0, 0, 0, 0,
            0, 1, 2, 0, 0, 0, 0, 0,
        ];
        let high_scores = parse(&buffer).unwrap();
        assert_eq!(high_scores.len(), 1);
        assert_eq!(high_scores[0].name, "JAM");
        assert_eq!(high_scores[0].score, 1113);

        let buffer = [
            9, 0, 12, 0, 89, 4, 0, 0,
            0, 1, 2, 0, 0, 0, 0, 0,
            0, 1, 2, 0,
        ];
        let error = parse(&buffer).unwrap_err();
        assert_eq!(error, "unparsed bytes at end");
    }

    #[test]
    fn test_load() {
        let mut file = NamedTempFile::new().unwrap();
        file.write_all(&[
            9, 0, 12, 0, 89, 4, 0, 0,
            0, 1, 2, 0, 0, 0, 0, 0,
            0, 1, 2, 0, 0, 0, 0, 0,
        ]).unwrap();
        let path = file.into_temp_path();
        let high_scores = load(&path).unwrap();
        assert_eq!(high_scores.len(), 1);
        assert_eq!(high_scores[0].name, "JAM");
        assert_eq!(high_scores[0].score, 1113);
    }
}
