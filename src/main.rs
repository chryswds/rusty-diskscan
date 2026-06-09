use std::env;
use std::io::Result;
use std::{fs, io};

pub fn readable_size(bytes: u64) -> String {
    let str_byte = bytes.to_string();

    let base: u64 = 1024;
    let float_base = base as f64;

    let base_power_two = base.pow(2);
    let float_base_power_two = base_power_two as f64;

    let float_value = bytes as f64;
    
    let base_power_three = base.pow(3);
    let float_base_power_three = base_power_three as f64;

    if bytes < base {
        let b = String::from("B");
        format!("{str_byte} {b}")
    } else if bytes < base.pow(2) {


        let kb_float_value = float_value / float_base;
        let kb = String::from("KB");
        format!("{kb_float_value:.1} {kb}")



    } else if bytes < base_power_three{
        let mb_float_value = float_value / float_base_power_two;
        let mb = String::from("MB");
        format!("{mb_float_value:.1} {mb}")



    } else {
        let gb_float_value = float_value / float_base_power_three;

        let gb = String::from("GB");
        format!("{gb_float_value:.1} {gb}")
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut entries = fs::read_dir(file_path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>>>()?;

    entries.sort();

    println!("{:?}", file_path);

    let mut total_size: u64 = 0;

    for file in &entries {
        let metadata = fs::metadata(file)?;

        let size = metadata.len();

        let readable_size = readable_size(size);
        total_size += size;

        println!("{:?} - size = {}", file, readable_size);
    }

    let readable_total_size = readable_size(total_size);

    println!("Total size = {}", readable_total_size);

    Ok(())
}
