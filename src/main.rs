use std::error::Error;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read};
use std::path::Path;
mod bhswz;
use bhswz::{SwzReader, SwzWriter, get_swz_file_name};

const SWZ_PATH: &str = "C:/Program Files (x86)/Steam/steamapps/common/Brawlhalla/Game.swz";
const OUTPUT_PATH: &str = "C:/Program Files (x86)/Steam/steamapps/common/Brawlhalla/Game_out";

fn dump_swz() -> Result<(), Box<dyn Error>> {
    let output_path = Path::new(OUTPUT_PATH);
    fs::create_dir_all(&output_path)?;

    let file = File::open(SWZ_PATH)?;
    let reader = BufReader::new(file);

    let mut swz_reader = SwzReader::new(reader, 659849070)?;

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

fn repack_swz() -> Result<(), std::io::Error> {
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

fn main() {
    dump_swz().unwrap();
    repack_swz().unwrap();
    dump_swz().unwrap();
}
