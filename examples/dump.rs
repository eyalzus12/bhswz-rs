use bhswz::{SwzReader, get_swz_file_name};
use std::error::Error;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    const SWZ_KEY: u32 = 685729090; // changes every patch! see find_key.
    const SWZ_PATH: &str = "C:/Program Files (x86)/Steam/steamapps/common/Brawlhalla/Game.swz";
    const OUTPUT_PATH: &str = "C:/Program Files (x86)/Steam/steamapps/common/Brawlhalla/Game_out";

    let output_path = Path::new(OUTPUT_PATH);
    fs::create_dir_all(&output_path)?;

    let file = File::open(SWZ_PATH)?;
    let reader = BufReader::new(file);

    let mut swz_reader = SwzReader::new(reader, SWZ_KEY)?;

    let mut buf = Vec::new();
    while swz_reader.read_file(&mut buf)? {
        let file_content = str::from_utf8(&buf)?;
        if let Some(file_name) = get_swz_file_name(&file_content) {
            println!("found {file_name}");
            let path = output_path.join(file_name);
            fs::write(path, &buf)?;
        } else {
            println!("failed to figure out file name");
        }

        buf.clear();
    }

    Ok(())
}
