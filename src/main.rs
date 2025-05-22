use std::error::Error;
use std::fs::{self, File};
mod bhswz;
use bhswz::{SwzReader, get_swz_file_name};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("C:/Program Files (x86)/Steam/steamapps/common/Brawlhalla/Dynamic.swz")?;
    let mut reader = SwzReader::new(file, 659849070)?;

    let mut buf = Vec::new();
    while let Ok(_) = reader.read_file(&mut buf) {
        let file_content = str::from_utf8(&buf)?;
        if let Some(file_name) = get_swz_file_name(&file_content) {
            let path = format!(
                "C:/Program Files (x86)/Steam/steamapps/common/Brawlhalla/Dynamic/{file_name}"
            );
            fs::write(path, &buf)?;
        } else {
            println!("failed to figure out file name");
        }

        buf.clear();
    }

    Ok(())
}
