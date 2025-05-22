use super::SwzRandom;
use std::{
    io::{Read, Write},
    string::FromUtf8Error,
};

use flate2::read::ZlibDecoder;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SwzReaderInitError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("key checksum mismatch (expected {expected:?}, found {calculated:?})")]
    KeyChecksumMismatch { expected: u32, calculated: u32 },
}

#[derive(Error, Debug)]
pub enum SwzReaderReadError {
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error("file checksum mismatch (expected {expected:?}, found {calculated:?})")]
    FileChecksumMismatch { expected: u32, calculated: u32 },
    #[error("decompressed file size mismatch (expected {expected:?}, found {calculated:?})")]
    DecompressedFileSizeMismatch { expected: u32, calculated: u64 },
}

#[derive(Error, Debug)]
pub enum SwzReaderReadStringError {
    #[error(transparent)]
    ReadError(#[from] SwzReaderReadError),
    #[error(transparent)]
    DecodeError(#[from] FromUtf8Error),
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
            return Err(SwzReaderInitError::KeyChecksumMismatch {
                expected: checksum,
                calculated: calculated_checksum,
            });
        }

        return Ok(SwzReader {
            reader: reader,
            random: random,
        });
    }

    pub fn read_file<W: Write>(&mut self, writer: &mut W) -> Result<(), SwzReaderReadError> {
        let mut buf = [0u8; 4];
        self.reader.read_exact(&mut buf)?;
        let compressed_size = u32::from_be_bytes(buf) ^ self.random.next();
        self.reader.read_exact(&mut buf)?;
        let decompressed_size = u32::from_be_bytes(buf) ^ self.random.next();
        self.reader.read_exact(&mut buf)?;
        let checksum = u32::from_be_bytes(buf);

        let mut file_buf = Vec::with_capacity(compressed_size as usize);
        self.reader
            .by_ref()
            .take(compressed_size.into())
            .read_to_end(&mut file_buf)?;

        let calculated_checksum = decrypt_buffer(&mut file_buf[..], &mut self.random);
        if checksum != calculated_checksum {
            return Err(SwzReaderReadError::FileChecksumMismatch {
                expected: checksum,
                calculated: calculated_checksum,
            });
        }

        let mut zlib = ZlibDecoder::new(&file_buf[..]);
        let bytes_read = std::io::copy(&mut zlib, writer)?;
        if bytes_read != decompressed_size.into() {
            return Err(SwzReaderReadError::DecompressedFileSizeMismatch {
                expected: decompressed_size,
                calculated: bytes_read,
            });
        }

        return Ok(());
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

fn decrypt_buffer(buf: &mut [u8], random: &mut SwzRandom) -> u32 {
    let mut checksum = random.next();
    for i in 0..buf.len() {
        let xor = (random.next() >> (i % 16)) as u8;
        buf[i] ^= xor;
        checksum = checksum.rotate_right((i % 7 + 1) as u32) ^ (buf[i] as u32);
    }
    return checksum;
}
