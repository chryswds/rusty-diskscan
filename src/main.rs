use std::env;
use std::io::Result;
use std::path::{Path, PathBuf};
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
    let entries = fs::read_dir(dir_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>>>()?;
    let mut total_size: u64 = 0;
    for entry in &entries {
        let metadata = fs::metadata(entry)?;
        

        if metadata.is_dir() {
            let folder_size = dir_size(entry).unwrap_or(0);
            total_size += folder_size;
        } else {
            let size = metadata.len();
            total_size += size;
        }
    }
    Ok(total_size)
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let path = Path::new(&args[1]);
    println!("{}", path.display());

    let entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>>>()?;

    let mut entries_by_size: Vec<(u64, &PathBuf)> = Vec::new();


    for entry in &entries {
        let metadata = fs::metadata(entry)?;

        if metadata.is_dir() {
            let folder_size = dir_size(entry).unwrap_or(0);
            entries_by_size.push((folder_size, entry));
        } else {
            let file_size = metadata.len();
            entries_by_size.push((file_size, entry));
        }
    }

    
    entries_by_size.sort_by_key(|(s, _)| std::cmp::Reverse(*s));

    let mut total_size: u64 = 0;
    for (size, path) in &entries_by_size {
        println!("{} ---- {}", path.display(), readable_size(*size));
        total_size += size;
    
    }

    println!("{}", readable_size(total_size));
    Ok(())
}
