use bhswz::keyfinder::find_swz_key_in_swf;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

#[cfg(feature = "keyfinder")]
fn main() -> Result<(), Box<dyn Error>> {
    const BH_AIR_PATH: &str = "C:/Program Files (x86)/Steam/steamapps/common/Brawlhalla/BrawlhallaAir.swf";

    let file = File::open(BH_AIR_PATH)?;
    let reader = BufReader::new(file);
    let swf_buf = swf::decompress_swf(reader)?;
    let swf = swf::parse_swf(&swf_buf)?;

    match find_swz_key_in_swf(&swf) {
        Some(key) => println!("found swz key: {}", key),
        None => println!("key not found"),
    }

    Ok(())
}
