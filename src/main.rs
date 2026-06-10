use std::env;
use std::io::Result;
use std::path::Path;
use std::{fs, io};

fn readable_size(bytes: u64) -> String {
    let base = 1024;

    let float_value = bytes as f64;

    if bytes < base {
        format!("{bytes} B")
    } else if bytes < base.pow(2) {
        format!("{:.1} KB", float_value / base as f64)
    } else if bytes < base.pow(3) {
        format!("{:.1} MB", float_value / base.pow(2) as f64)
    } else {
        format!("{:.1} GB", float_value / base.pow(3) as f64)
    }
}

fn dir_size(dir_path: &Path) -> io::Result<u64> {
    let mut entries = fs::read_dir(dir_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>>>()?;

    entries.sort();

    println!("{:?}", dir_path);

    let mut total_size: u64 = 0;

    for entry in &entries {
        let metadata = fs::metadata(entry)?;

        let size = metadata.len();

        let readable_size = readable_size(size);
        total_size += size;

        println!("{:?} - size = {}", entry, readable_size);
    }

    let readable_total_size = readable_size(total_size);

    println!("Total size = {}", readable_total_size);

    Ok(total_size)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let dir_path_str = &args[1];
    let dir_path_path = Path::new(&dir_path_str);

    let _total_size = dir_size(dir_path_path)?;

    Ok(())
}
