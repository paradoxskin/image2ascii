use image::GenericImageView;
use termion::color;
use termion::cursor;
use std::io::{Stdout, BufWriter, Write};
use std::collections::VecDeque;
use std::time;
use std::sync::Mutex;
use std::sync::Arc;

pub struct Show {
	screen_weight: u32,
	screen_height: u32,
	now: Vec<Vec<Node>>,
}

impl Show {
	const FPS: u8 = 30;
	pub fn init() -> Self {
		let (a, b) = termion::terminal_size().unwrap();
		let (screen_weight, screen_height) = (a as u32, b as u32);
		let mut now: Vec<Vec<Node>> = Vec::new();
		for _ in 0..screen_height {
			let mut tmp_vec = Vec::<Node>::new();
			for _ in 0..screen_weight {
				tmp_vec.push(Node::init((0, 0, 0), 0, (2333, 2333)));
			}
			now.push(tmp_vec);
		}
		Show {
			screen_weight,
			screen_height,
			now,
		}
	}
	pub fn get_size(&self) -> (u32, u32) {
		(self.screen_weight, self.screen_height)
	}

	pub fn run(&mut self, imgs: &Read) {
		let wait: f64 = 1.0 / (Self::FPS as f64);
		let stdout = std::io::stdout();
		let mut writer = BufWriter::new(stdout);
		print!("{}{}", termion::clear::All, cursor::Hide);
		loop {
			let begin = time::Instant::now();
			if self.play_next_frame(&mut writer, imgs) {
				break;
			}
			writer.flush().unwrap();
			let pass = time::Instant::now().duration_since(begin).as_secs_f64();
			std::thread::sleep(time::Duration::from_secs_f64(wait - pass));
		}
		println!("{}{}{}", cursor::Show, color::Bg(color::Reset), color::Fg(color::Reset));
	}

	fn play_next_frame(&mut self, writer: &mut BufWriter<Stdout>, imgs: &Read) -> bool {
		let opt_next_frame = imgs.get_fron();
		if let Some(next_frame) = opt_next_frame {
			for x in 0..next_frame[0].len() {
				for y in 0..next_frame.len() {
					if self.now[y][x] == next_frame[y][x] {
						continue;
					}
					self.now[y][x] = next_frame[y][x].clone();
					self.now[y][x].write_pixel(writer);
				}
			}
			return false;
		}
		return true;
	}
}

#[derive(PartialEq, Eq, Clone)]
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

	pub fn write(&self, writer: &mut BufWriter<Stdout>) {
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

pub struct Read {
	cargo: Arc<Mutex<VecDeque<Vec<Vec<Node>>>>>,
}

impl Read {
	pub fn init() -> Self {
		let cargo: Arc<Mutex<VecDeque<Vec<Vec<Node>>>>> = Arc::new(Mutex::new(VecDeque::new()));
		Self {
			cargo
		}
	}

	pub fn add_back(&self, frame: Vec<Vec<Node>>) {
		let cp = self.cargo.clone();
		let mut dp = cp.lock().unwrap();
		dp.push_back(frame);
	}
	pub fn get_fron(&self) -> Option<Vec<Vec<Node>>> {
		let cp = self.cargo.clone();
		let mut dp = cp.lock().unwrap();
		dp.pop_front()
	}

	pub fn read_from_img(img: image::DynamicImage, screen_size: (u32, u32)) -> Vec<Vec<Node>> {
		let mut vec: Vec<Vec<Node>> = Vec::new();
		let (weight, height) = screen_size;
		//let (img_w, img_h) = (img.width(), img.height() * 2 / 3);
		let (img_w, img_h) = (img.width(), img.height() / 2);
		let mut ww = weight;
		let mut hh = img_h * weight / img_w;
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
	pub fn img_file(filename: &str) -> image::DynamicImage {
		let image = image::open(filename).unwrap();
		image
	}
}
