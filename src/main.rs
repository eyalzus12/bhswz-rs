use std::error::Error;
use std::fs::{self, File};
mod bhswz;
use bhswz::{SwzRandom, SwzReader};

fn main() -> Result<(), Box<dyn Error>> {
    let mut random = SwzRandom::new(0);
    for _ in 0..100 {
        println!("{}", random.next());
    }

    let file = File::open("C:/Program Files (x86)/Steam/steamapps/common/Brawlhalla/Dynamic.swz")?;
    let mut reader = SwzReader::new(file, 659849070)?;

    let mut buf = Vec::new();
    while let Ok(_) = reader.read_file(&mut buf) {
        let name = random.next().to_string();
        let path =
            format!("C:/Program Files (x86)/Steam/steamapps/common/Brawlhalla/Dynamic/{name}.xml");
        fs::write(path, &buf)?;

        buf.clear();
    }

    Ok(())
}
