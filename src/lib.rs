use std::error::Error;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Read};
use std::path::Path;
mod bhswz;
use bhswz::{SwzReader, SwzWriter, get_swz_file_name};

/// Dumps the contents of an SWZ file to the specified output directory.
/// ### Args
/// * `swz_path` - The path of the SWZ file to be decrypted.
/// * `output_path` - The directory where the contents will be dumped.
/// * `key` - The decryption key for the SWZ file. It can be found in BrawlhallaAir.swf and changes with each patch.
/// ### Returns
/// `Ok(())` on success, or an error if the file extraction fails.
pub fn dump_swz(swz_path: &str, output_path: &str, key: u32) -> Result<(), Box<dyn Error>> {
    let output_path = Path::new(output_path);
    fs::create_dir_all(&output_path)?;

    let file = File::open(swz_path)?;
    let reader = BufReader::new(file);
    let mut swz_reader = SwzReader::new(reader, key)?;

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

/// Repackages the contents of an unpacked folder into a SWZ file.
/// ### Args
/// * `unpacked_folder_path` - The path to the folder containing unpacked files.
/// * `swz_path` - The path where the SWZ file will be created/overwritten.
/// * `key` - The decryption key for the SWZ file. It can be found in BrawlhallaAir.swf and changes with each patch.
/// ### Returns
/// `Ok(())` on success, or an error if the file packing fails.
pub fn repack_swz(unpacked_folder_path: &str, swz_path: &str, key: u32) -> Result<(), std::io::Error> {
    let new_swz = BufWriter::new(File::create(swz_path)?);
    let mut swz_writer = SwzWriter::new(new_swz, key, 0)?;

    let mut buf = Vec::new();
    for entry in fs::read_dir(unpacked_folder_path)? {
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
