mod utils;

use utils::{Show, Readd};
use std::sync::{Arc, Mutex};
use rodio::Sink;
use std::io::Read;

fn main() {
	let picts_arc = Arc::new(Mutex::new(Readd::init()));
	let mut show: Show = Show::init();
	let size = show.get_size();
	let picts = picts_arc.clone();
	/*
	std::thread::spawn(move ||{
		let mut tmp: Vec<Vec<Vec<utils::Node>>> = Vec::new();
		for i in 1..(6570) {
			let filename = format!("/tmp/image-{:04}.jpg", i);
			let image = Readd::img_file(&filename);
			let nodes = Readd::read_from_img(image, size);
			tmp.push(nodes);
			println!("{}", i);
		}
		utils::Readd::intobin(tmp, "ttmp");
	}).join().unwrap();
	*/
	let (s, r) = std::sync::mpsc::channel::<i32>();
	std::thread::spawn(move || {
		let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
		let file = std::fs::File::open("/home/paradoxd/boring/bad_apple/audio.mp3").unwrap();
		let source = rodio::Decoder::new(file).unwrap();
		let sink = Sink::try_new(&stream_handle).unwrap();
		r.recv().unwrap();
		sink.append(source);
		std::thread::sleep(std::time::Duration::from_secs(2000));
	});
	{
		let tmp = utils::Readd::read_from_bin("/home/paradoxd/boring/bad_apple/ttmp");
		let tmpp = picts.lock().unwrap();
		for i in tmp {
			tmpp.add_back(i);
		}
	}
	println!("ok");
	s.send(233).unwrap();
	show.run(picts_arc);
}
