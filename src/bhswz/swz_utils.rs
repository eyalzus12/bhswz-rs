use super::SwzRandom;

pub(super) fn calculate_key_checksum(key: u32, random: &mut SwzRandom) -> u32 {
    let mut checksum = 0x2DF4A1CDu32;
    let rounds = key % 31 + 5;
    for _ in 0..rounds {
        checksum ^= random.next();
    }
    return checksum;
}

pub(super) fn decrypt_buffer(buf: &mut [u8], random: &mut SwzRandom) -> u32 {
    let mut checksum = random.next();
    for i in 0..buf.len() {
        buf[i] ^= get_xor(random, i);
        checksum = update_checksum(checksum, buf[i], i);
    }
    return checksum;
}

pub(super) fn encrypt_buffer(buf: &mut [u8], random: &mut SwzRandom) -> u32 {
    let mut checksum = random.next();
    for i in 0..buf.len() {
        checksum = update_checksum(checksum, buf[i], i);
        buf[i] ^= get_xor(random, i);
    }
    return checksum;
}

fn get_xor(random: &mut SwzRandom, i: usize) -> u8 {
    (random.next() >> (i % 16)) as u8
}

fn update_checksum(checksum: u32, byte: u8, i: usize) -> u32 {
    checksum.rotate_right((i % 7 + 1) as u32) ^ (byte as u32)
}
