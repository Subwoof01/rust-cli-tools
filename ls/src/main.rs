use std::{path::Path, fs, os::unix::fs::PermissionsExt};
use clap::Parser;
use colored::{Colorize, ColoredString};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The path to list.
    #[arg(default_value = ".")]
    path: String,
}

fn main() {
    // Get the path through arguments (if it exists)
    let args = Args::parse();

    let dir_entries = get_dir_entries(&args.path);

    match dir_entries {
        Ok(dir_entries) => {
            for dir_entry in dir_entries {
                // let file_name = dir_entry.file_name();
                // println!("FILE NAME: {:?}", file_name);
                // let dir = dir_entry.path();
                // println!("PATH: {:?}", dir);
                let metadata = dir_entry.metadata().unwrap();
                let mut name = dir_entry.file_name().into_string().unwrap().cyan();
                let perms_bits = metadata.permissions().mode();
                let mut permissions = "";

                let is_dir = metadata.is_dir();

                if is_dir {
                    name = name.bold().blue().bold();
                    print!("d");
                }
                else {
                    print!("-");
                }

                let oct = format!("{:#o}", perms_bits);
                let bits = {
                    let r = oct.char_indices().nth_back(2).unwrap().0;
                    &oct[r..]
                };

                let mut perms = String::new();
                for c in bits.chars() {
                    match c {
                        '0' => { perms = format!("{perms}---") },
                        '1' => { perms = format!("{perms}--x") },
                        '2' => { perms = format!("{perms}-w-") },
                        '3' => { perms = format!("{perms}-wx") },
                        '4' => { perms = format!("{perms}r--") },
                        '5' => { perms = format!("{perms}r-x") },
                        '6' => { perms = format!("{perms}rw-") },
                        '7' => { perms = format!("{perms}rwx") },
                        _ => { perms = format!("{perms}---") },                   
                    }
                }

                print!("{} ", perms);
                print!("{}", name);
                println!();
            }
        }
        _ => { }
    }

}

fn get_dir_entries(read_dir_path: &str) -> Result<Vec<fs::DirEntry>, std::io::Error> {
    let mut dir_entries = vec![];

    for dir_entry in fs::read_dir(read_dir_path)? {
        let de = dir_entry?;
        dir_entries.push(de);
    }

    Ok(dir_entries)
}
