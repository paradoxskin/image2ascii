mod utils;

use utils::{Show, Readd};
use std::sync::{Arc, Mutex};
use rodio::Sink;
use std::io::Read;

fn main() {
	let picts_arc = Arc::new(Mutex::new(Readd::init()));
	let screen = Arc::new(Mutex::new(utils::Show::init()));
	let timeline = utils::TimeLine::init();
	let size = timeline.get_size();
	let picts = picts_arc.clone();
	let z = Readd::read_frame(String::from("/home/paradoxd/Music/Video/solBadguy.mp4"), size);
	println!("{}", z.len());
	/*
	let mut tmp: Vec<Vec<Vec<utils::Node>>> = Vec::new();
	let arcc: Arc<Mutex<Vec<Vec<Vec<utils::Node>>>>> = Arc::new(Mutex::new(Vec::new()));
	{
		let mut ac = arcc.lock().unwrap();
		for _ in 1..3348 {
			let ttmp: Vec<Vec<utils::Node>> = Vec::new();
			ac.push(ttmp);
		}
	}
	let mut handles: Vec<std::thread::JoinHandle<()>> = Vec::new();
	let readmutex = Arc::new(std::sync::RwLock::new(4));
	for i in 1..(3348) {
		loop {
			let p = readmutex.read().unwrap();
			if *p > 0 {
				break;
			}
		}
		let zz = arcc.clone();
		let cc = readmutex.clone();
		{
			let mut tt = cc.write().unwrap();
			*tt -= 1;
		}
		let handle = std::thread::spawn(move || {
				let filename = format!("/tmp/image-{}.jpg", i);
				let image = Readd::img_file(&filename);
				let nodes = Readd::read_from_img(image, size);
				{
					let mut bb = zz.lock().unwrap();
					bb[i - 1] = nodes;
				}
				{
					let mut tt = cc.write().unwrap();
					*tt += 1;
				}
				println!("{}", i);
		});
		handles.push(handle);
	}
	for handle in handles {
		handle.join().unwrap();
	}
	{
		let ac = arcc.lock().unwrap();
		for i in ac.iter() {
			tmp.push(i.clone());
		}
	}
	utils::Readd::intobin(tmp, "ttmp");
	*/
	/*
	let (s, r) = std::sync::mpsc::channel::<i32>();
	std::thread::spawn(move || {
		let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
		let file = std::fs::File::open("/home/paradoxd/boring/sol_badguy/audio.mp3").unwrap();
		let source = rodio::Decoder::new(file).unwrap();
		let sink = Sink::try_new(&stream_handle).unwrap();
		r.recv().unwrap();
		sink.append(source);
		std::thread::sleep(std::time::Duration::from_secs(2000));
	});
	{
		let tmp = utils::Readd::read_from_bin("/home/paradoxd/boring/sol_badguy/ttmp");
		let tmpp = picts.lock().unwrap();
		for i in tmp {
			tmpp.add_back(i);
		}
	}
	println!("ok");
	s.send(233).unwrap();
	show.run(picts_arc);
	*/
}
