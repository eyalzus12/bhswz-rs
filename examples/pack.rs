use bhswz_rs::SwzWriter;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufWriter, Read};

fn main() -> Result<(), Box<dyn Error>> {
    const SWZ_PATH: &str = "C:/Program Files (x86)/Steam/steamapps/common/Brawlhalla/Game.swz";
    const OUTPUT_PATH: &str = "C:/Program Files (x86)/Steam/steamapps/common/Brawlhalla/Game_out";

    let new_swz = BufWriter::new(File::create(SWZ_PATH)?);
    let mut swz_writer = SwzWriter::new(new_swz, 659849070, 0)?;

    let mut buf = Vec::new();
    for entry in fs::read_dir(OUTPUT_PATH)? {
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
