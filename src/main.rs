mod player;
mod reader;
mod color;
mod data;

use std::env;

pub const HELP: &str = "Usage of image2ascii:
        -f file_path [output_file_path] (default under same dir)
        -p play_file_path";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("{}", HELP);
        return;
    }
    match args[1].as_str() {
        "-f" => {
            let opt_in_path = args.get(2);
            if let Some(in_path)  = opt_in_path {
                let file_path = in_path.to_owned();

                let out_path: String;

                let opt_out_path = args.get(3);
                if let Some(path) = opt_out_path {
                    out_path = path.to_owned();
                }
                else {
					out_path = format!("{}.i2a", file_path);
                }

				if !std::path::Path::new(&file_path).exists() {
					println!("Path Not Exists!\n\n{}", HELP);
					return;
				}

                crate::reader::process(&file_path, &out_path);
            }
            else {
                println!("Need Path!\n\n{}", HELP);
            }
        },
        "-p" => {
            let get_from_vec = args.get(2);
            if let Some(path)  = get_from_vec {
                let file_path = path.to_owned();

                crate::player::play(&file_path);
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
