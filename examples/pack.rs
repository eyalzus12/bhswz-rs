use bhswz::SwzWriter;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufWriter, Read};

fn main() -> Result<(), Box<dyn Error>> {
    const SWZ_KEY: u32 = 685729090; // changes every patch! see find_key.
    const SWZ_SEED: u32 = 0; // random
    const FOLDER_PATH: &str = "C:/Program Files (x86)/Steam/steamapps/common/Brawlhalla/Game_out";
    const SWZ_PATH: &str = "C:/Program Files (x86)/Steam/steamapps/common/Brawlhalla/Game.swz";

    let new_swz = BufWriter::new(File::create(SWZ_PATH)?);
    let mut swz_writer = SwzWriter::new(new_swz, SWZ_KEY, SWZ_SEED)?;

    let mut buf = Vec::new();
    for entry in fs::read_dir(FOLDER_PATH)? {
        let path = entry.unwrap().path();
        if !path.is_file() {
            continue;
        }
        println!("inserting {:?}", path);

        let mut file = File::open(path)?;
        file.read_to_end(&mut buf)?;
        drop(file);

        swz_writer.write_file(&buf)?;

        buf.clear();
    }

    Ok(())
}
