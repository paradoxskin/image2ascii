use image::GenericImageView;
use ffmpeg_next as ffmpeg;
use termion::color;
use termion::cursor;
use serde_derive::{Serialize, Deserialize};

use std::io::{Stdout, BufWriter, Write, Read};
use std::collections::VecDeque;
use std::time;
use std::sync::Mutex;
use std::sync::Arc;

pub struct TimeLine {
	screen_width: u32,
	screen_height: u32,
}

impl TimeLine {
	const FPS: u8 = 30;
	pub fn init() -> Self {
		let (a, b) = termion::terminal_size().unwrap();
		println!("{}少女祈祷中...", termion::clear::All);
		Self {
			screen_width: a as u32,
			screen_height: b as u32,
		}
	}

	pub fn get_size(&self) -> (u32, u32) {
		(self.screen_width, self.screen_height)
	}

	pub fn run(&mut self, imgs: Arc<Mutex<Readd>>, screen: Arc<Mutex<Show>>) {
		let wait: f64 = 1.0 / (Self::FPS as f64);
		let stdout = std::io::stdout();
		let writer = Arc::new(Mutex::new(BufWriter::new(stdout)));
		print!("{}{}", termion::clear::All, cursor::Hide);
		loop {
			let begin = time::Instant::now();
			if self.play_next_frame_pro(writer.clone(), imgs.clone(), screen.clone()) {
				break;
			}
			let pass = time::Instant::now().duration_since(begin).as_secs_f64();
			let dis = wait - pass;
			if dis > 0. {
				std::thread::sleep(time::Duration::from_secs_f64(wait - pass));
			}
		}
		println!("{}{}{}", cursor::Show, color::Bg(color::Reset), color::Fg(color::Reset));
	}

	fn play_next_frame_pro(&mut self, writer: Arc<Mutex<BufWriter<Stdout>>>, imgs: Arc<Mutex<Readd>>, screen: Arc<Mutex<Show>>) -> bool {
		let imgs = imgs.lock().unwrap();
		let opt_next_frame = imgs.get_fron();
		if let Some(next_frame) = opt_next_frame {
			std::thread::spawn(move||{
				let mut writer = writer.lock().unwrap();
				let mut screen = screen.lock().unwrap();
				for upd in next_frame {
					let (x, y) = upd.pos;
					let (x, y) = (x as usize, y as usize);
					screen.now[y][x] = upd.clone();
					screen.now[y][x].write(&mut writer);
				}
				writer.flush().unwrap();
			});
			return false;
		}
		return true;
	}
}

pub struct Show {
	now: Vec<Vec<Node>>,
}

impl Show {
	pub fn init() -> Self {
		let (a, b) = termion::terminal_size().unwrap();
		let mut now: Vec<Vec<Node>> = Vec::new();
		for _ in 0..b {
			let mut tmp_vec = Vec::<Node>::new();
			for _ in 0..a {
				tmp_vec.push(Node::init((0, 0, 0), 0, (0, 0)));
			}
			now.push(tmp_vec);
		}
		Show {
			now,
		}
	}
}

#[derive(PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Node {
	node_col: (u8, u8, u8),
	pub node_style: u8,
	pos: (u32, u32),
}

impl Node {
	const CHAR: [char; 16] = [' ', '.', ':', ';', 'i', '7', 'r', 'a', 'Z', 'S', '0', '8', 'X', 'M', 'W', '@'];
	pub fn init(node_col: (u8, u8, u8), node_style: u8, pos: (u32, u32)) -> Self {
		Node {
			node_col,
			node_style,
			pos
		}
	}

	pub fn write(&self, writer: &mut std::sync::MutexGuard<BufWriter<Stdout>>) {
		write!(writer, "{}{}{}",
				cursor::Goto(self.pos.0 as u16, self.pos.1 as u16),
				color::Fg(color::Rgb(self.node_col.0, self.node_col.1, self.node_col.2)),
				Self::CHAR[self.node_style as usize]).unwrap();
	}

	pub fn write_pixel(&self, writer: &mut BufWriter<Stdout>) {
		write!(writer, "{}{} ",
				cursor::Goto(self.pos.0 as u16, self.pos.1 as u16),
				color::Bg(color::Rgb(self.node_col.0, self.node_col.1, self.node_col.2))).unwrap();
	}
}

pub struct Readd {
	cargo: Arc<Mutex<VecDeque<Vec<Node>>>>,
}

impl Readd {
	pub fn init() -> Self {
		let cargo: Arc<Mutex<VecDeque<Vec<Node>>>> = Arc::new(Mutex::new(VecDeque::new()));
		Self {
			cargo
		}
	}

	pub fn add_back(&self, frame: Vec<Node>) {
		let cp = self.cargo.clone();
		let mut dp = cp.lock().unwrap();
		dp.push_back(frame);
	}

	pub fn get_fron(&self) -> Option<Vec<Node>> {
		let cp = self.cargo.clone();
		let mut dp = cp.lock().unwrap();
		dp.pop_front()
	}

	/// static function start

	pub fn read_from_img(img: image::DynamicImage, screen_size: (u32, u32)) -> Vec<Vec<Node>> {
		let mut vec: Vec<Vec<Node>> = Vec::new();
		let (width, height) = screen_size;
		//let (img_w, img_h) = (img.width(), img.height() * 2 / 3);
		let (img_w, img_h) = (img.width(), img.height() / 2);
		let mut ww = width;
		let mut hh = img_h * width / img_w;
		if hh > height {
			hh = height;
			ww = img_w * height / img_h;
		}
		let col_img = img.resize_exact(ww, hh, image::imageops::FilterType::Triangle);
		let luma_img = img.resize_exact(ww, hh, image::imageops::FilterType::Triangle).into_luma8();
		for y in 0..hh {
			let mut tmp_vec: Vec<Node> = Vec::new();
			for x in 0..ww {
				let dep = luma_img.get_pixel(x, y).0[0] / 16;
				let col = col_img.get_pixel(x, y).0;
				tmp_vec.push(Node::init((col[0], col[1], col[2]), dep, (x + 1, y + 1)));
			}
			vec.push(tmp_vec);
		}
		return vec;
	}

	pub fn smallize(video: Vec<Vec<Vec<Node>>>) -> Vec<Vec<Node>>{
		let mut small: Vec<Vec<Node>> = Vec::new();
		let mut first_frame: Vec<Node> = Vec::new();
		let (width, height) = (video[0][0].len(), video[0].len());
		if video.len() > 0 {
			for i in &video[0] {
				for node in i {
					first_frame.push(node.clone());
				}
			}
		}
		small.push(first_frame);
		for idx in 1..video.len() {
			let mut now_frame: Vec<Node> = Vec::new();
			for y in 0..height {
				for x in 0..width {
					if video[idx][y][x] == video[idx - 1][y][x] {
						continue;
					}
					now_frame.push(video[idx][y][x].clone());
				}
			}
			small.push(now_frame);
		}
		small
	}

	pub fn img_file(filename: &str) -> image::DynamicImage {
		let image = image::open(filename).unwrap();
		image
	}

	pub fn read_from_video(filename: &str) -> Vec<Vec<Vec<Node>>> {
		ffmpeg::init().unwrap();
		let images: Vec<Vec<Vec<Node>>> = Vec::new();
		let filename = String::from(filename);
		let format = ffmpeg::format::input(&filename).unwrap();
		let steams = format
			.streams()
			.best(ffmpeg::media::Type::Video)
			.unwrap();
		let tmp = steams.index();
		images
	}

	pub fn intobin(video: Vec<Vec<Vec<Node>>>, filename: &str) {
		let mut file = std::fs::File::create(filename).unwrap();
		let bytes = bincode::serialize(&video).unwrap();
		file.write_all(&bytes).unwrap();
	}

	pub fn read_from_bin(filename: &str) -> Vec<Vec<Vec<Node>>> {
		let file = std::fs::File::open(filename).unwrap();
		let reader = std::io::BufReader::new(file);
		let data: Vec<Vec<Vec<Node>>> = bincode::deserialize_from(reader).unwrap();
		data
	}
}
