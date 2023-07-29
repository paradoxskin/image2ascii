mod utils;
mod player;
mod reader;
mod color;
mod data;

use std::env;
use std::fs;

pub const HELP: &str = "Usage of image2ascii:
        -f file_path [output_dir_path] (default under same dir)
        -p play_file_path";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("{}", HELP);
        return;
    }
    match args[1].as_str() {
        "-f" => {
            let get_from_vec = args.get(2);
            if let Some(path)  = get_from_vec {
                let file_path = path.as_str();
                let out_path: &str;

                let get_from_vec = args.get(3);
                if let Some(path) = get_from_vec {
                    out_path = path;
                }
                else {
					let parent = std::path::Path::new(file_path).parent();
					if let Some(pa) = parent {
						if let Some(past) = pa.to_str() {
							out_path = past;
						}
						else {
							println!("Can't Set the Default Dir!\n\n{}", HELP);
							return;
						}
					}
					else {
						println!("Can't Set the Default Dir!\n\n{}", HELP);
						return;
					}
                }

				if !std::path::Path::new(file_path).exists() {
					println!("Path Not Exists!\n\n{}", HELP);
					return;
				}
				if !std::path::Path::new(out_path).exists() {
					println!("Parent Dir Not Exists or Can't Open!\n\n{}", HELP);
					return;
				}

                crate::reader::process(file_path, out_path);
            }
            else {
                println!("Need Path!\n\n{}", HELP);
            }
        },
        "-p" => {
            let get_from_vec = args.get(2);
            if let Some(path)  = get_from_vec {
                let file_path = path;

                crate::player::play(file_path);
            }
            else {
                println!("Need Play Path!\n\n{}", HELP);
            }
        },
        _ => {
            println!("{}", HELP);
        }
    }
}
