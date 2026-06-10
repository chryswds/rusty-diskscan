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
            let folder_size = dir_size(entry)?;
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
    let dir_path_str = &args[1];
    let dir_path_path = Path::new(&dir_path_str);
    println!("{:?}", dir_path_path);

    let entries = fs::read_dir(dir_path_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>>>()?;

    let mut v: Vec<(u64, &PathBuf)> = Vec::new();


    for entry in &entries {
        let metadata = fs::metadata(entry)?;

        if metadata.is_dir() {
            let folder_size = dir_size(entry)?;
            v.push((folder_size, entry));
        } else {
            let file_size = metadata.len();
            v.push((file_size, entry));
        }
    }

    v.sort_by(|a, b| b.cmp(a));
   

    let mut total_size: u64 = 0;
    for (size, path) in &v {
        println!("{} ---- {}", path.display(), readable_size(*size));
        total_size += size;
    
    }

    println!("{}", readable_size(total_size));
    Ok(())
}
