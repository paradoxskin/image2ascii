mod utils;
mod player;
mod reader;
mod color;
mod data;

use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let help = "Usage of image2ascii:
        -f file_path [output_dir_path] (default under same dir)
        -p play_file_path
        ";
    if args.len() == 1 {
        println!("{}", help);
        return;
    }
    match args[1].as_str() {
        "-f" => {
            let get_from_vec = args.get(2);
            if let Some(path)  = get_from_vec {
                let file_path = path;
                let out_path: &str;

                let get_from_vec = args.get(3);
                if let Some(path) = get_from_vec {
                    out_path = path;
                }
                else {
					let parent = std::path::Path::new(&file_path).parent();
					if let Some(pa) = parent {
						if let Some(past) = pa.to_str() {
							out_path = past;
						}
						else {
							println!("Can't Set the Default Dir!\n\n{}", help);
							return;
						}
					}
					else {
						println!("Can't Set the Default Dir!\n\n{}", help);
						return;
					}
                }

				if std::path::Path::new(file_path).exists() {
					println!("Path Not Exists!\n\n{}", help);
					return;
				}
				if std::path::Path::new(out_path).exists() {
					println!("Parent Dir Not Exists or Can't Open!\n\n{}", help);
					return;
				}

                // TODO start process
            }
            else {
                println!("Need Path!\n\n{}", help);
            }
        },
        "-p" => {
            let get_from_vec = args.get(2);
            if let Some(path)  = get_from_vec {
                let file_path = path;
                // TODO play the video
            }
            else {
                println!("Need Play Path!\n\n{}", help);
            }
        },
        _ => {
            println!("{}", help);
        }
    }
}
