use super::{SwzRandom, calculate_key_checksum, encrypt_buffer};
use flate2::{Compression, write::ZlibEncoder};
use std::io::Write;

pub struct SwzWriter<W> {
    writer: W,
    random: SwzRandom,
}

impl<W: Write> SwzWriter<W> {
    pub fn new(mut writer: W, key: u32, seed: u32) -> Result<SwzWriter<W>, std::io::Error> {
        let mut random = SwzRandom::new(key ^ seed);
        let checksum = calculate_key_checksum(key, &mut random);
        writer.write_all(&u32::to_be_bytes(checksum))?;
        writer.write_all(&u32::to_be_bytes(seed))?;
        Ok(Self {
            writer: writer,
            random: random,
        })
    }

    pub fn write_file(&mut self, file_content: &[u8]) -> Result<(), std::io::Error> {
        let decompressed_size: u32 = file_content.len().try_into().unwrap();

        let compressed_size_xor: u32 = self.random.next();
        let decompressed_size_xor = self.random.next();

        /*
        because the checksum is written before the file,
        we have to encrypt the buffer before writing it.
        so we have to store it in memory.
        */
        let mut intermediate = Vec::new();
        let mut encoder = ZlibEncoder::new(&mut intermediate, Compression::best());
        encoder.write_all(file_content)?;
        drop(encoder); // drop so we can access intermediate again

        let compressed_size: u32 = intermediate.len().try_into().unwrap();

        let checksum = encrypt_buffer(&mut intermediate, &mut self.random);

        self.writer
            .write_all(&u32::to_be_bytes(compressed_size ^ compressed_size_xor))?;
        self.writer
            .write_all(&u32::to_be_bytes(decompressed_size ^ decompressed_size_xor))?;
        self.writer.write_all(&u32::to_be_bytes(checksum))?;
        self.writer.write_all(&intermediate)?;

        Ok(())
    }
}
