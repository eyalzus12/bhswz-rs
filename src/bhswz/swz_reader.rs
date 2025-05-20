use super::SwzRandom;
use std::io::Read;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SwzReaderInitError {
    #[error("transparent")]
    IOError(#[from] std::io::Error),
    #[error("key checksum mismatch (expected {expected:?}, found {calculated:?})")]
    InvalidKeyChecksum { expected: u32, calculated: u32 },
}

pub struct SwzReader<R> {
    reader: R,
    random: SwzRandom,
}

impl<R> SwzReader<R>
where
    R: Read,
{
    pub fn new(mut reader: R, key: u32) -> Result<SwzReader<R>, SwzReaderInitError> {
        let mut buf = [0u8; 4];
        reader.read_exact(&mut buf)?;
        let checksum = u32::from_be_bytes(buf);
        reader.read_exact(&mut buf)?;
        let seed = u32::from_be_bytes(buf);

        let mut random = SwzRandom::new(key ^ seed);
        let calculated_checksum = calculate_key_checksum(key, &mut random);

        if checksum != calculated_checksum {
            return Err(SwzReaderInitError::InvalidKeyChecksum {
                expected: checksum,
                calculated: calculated_checksum,
            });
        }

        return Ok(SwzReader {
            reader: reader,
            random: random,
        });
    }
}

fn calculate_key_checksum(key: u32, random: &mut SwzRandom) -> u32 {
    let mut checksum = 0x2DF4A1CDu32;
    let rounds = key % 31 + 5;
    for _ in 0..rounds {
        checksum ^= random.next();
    }
    return checksum;
}
