mod utils;

use utils::{Show, Readd};
use std::sync::{Arc, Mutex};
use rodio::Sink;
use std::io::Read;
use std::env;

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
        // TODO check usable of path
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
                    // TODO get parent dir
                    out_path = "";
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
