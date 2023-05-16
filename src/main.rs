use image;
use image::GenericImageView;
use termion;
use termion::color;

fn main() {
	// read in a image
	let mut tmp = image::io::Reader::open("/home/paradox/Pictures/Akira-Kurusu-Anime-Persona-5-4k.jpg").unwrap().decode().unwrap();
	let (s_w, s_h) = termion::terminal_size().unwrap();
	//let s_h = (s_h * 3 / 2) as u32;
	let s_h = s_h as u32;
	let s_w = s_w as u32;
	let w = tmp.width();
	let h = tmp.height() * 2 / 3;
	let mut ww = s_w;
	let mut hh = h * s_w / w;
	if hh > s_h {
		hh = s_h;
		ww = w * s_h / h;
	}
	let xtmp = tmp.resize_exact(ww, hh, image::imageops::FilterType::Triangle);
	let ttmp = tmp.resize_exact(ww, hh, image::imageops::FilterType::Triangle);
	let xxtmp = xtmp.into_luma8();
	xxtmp.save("tmp.png").unwrap();
	//let ascii = "@&%$#~=`'!;:,.  ";
	//let ascii = " .,:;!'`~=$&%#@ ";
	let ascii = " .,:;i7raZS08XMW@ ";
	println!("");
	for y in 0..hh {
		for x in 0..ww {
			let tmp = xxtmp.get_pixel(x, y);
			let dep = (tmp.0[0] / 16) as usize;
			let col = ttmp.get_pixel(x, y).0;
			print!("{}{}{}", color::Fg(color::Rgb(col[0], col[1], col[2])), &ascii[dep..(dep + 1)], color::Fg(color::Reset));
		}
		println!();
	}
	println!("{} {}", s_w, s_h);
}

struct Show {
	
}

struct Read {
	
}
