use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::Path;

use sha2::Digest;
use sha2::Sha256;

pub fn sha256_digest<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
    let file = File::open(path)?;

    let mut reader = BufReader::new(file);
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 4096];

    loop {
        let bytes_read = reader.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        #[expect(clippy::indexing_slicing, reason = "invariant is upheld by the reader")]
        hasher.update(&buffer[..bytes_read]);
    }

    Ok(hex::encode(hasher.finalize()))
}
