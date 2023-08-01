pub mod tools {
    use crate::player::Node;
    use crate::color::Color;
    use image::GenericImageView;
    use std::collections::HashSet;

    fn rgb2dep(r: u8, g: u8, b: u8) -> u8 {
		let dep = (r as f64 * 0.3 + 0.59 * g as f64 + 0.11 * b as f64) as u8;
        dep
    }
    fn rgb2my256(r: u8, g: u8, b: u8) -> u8 {
        r / 43 * 36 + g / 43 * 6 + b / 43
    }

    fn rgb2df256(r: u8, g: u8, b: u8) -> u8 {
        r / 32 * 32 + g / 32 * 4 + b / 64
    }

    /// color Option:
    /// 0 -> RGB
    /// 1 -> My256
    /// 2 -> Df256
    pub fn img2asc(img: image::DynamicImage, width: u16, height: u16, color_flag: u8) -> Vec<Node> {
        let mut asc: Vec<Node> = Vec::new();
		let (img_w, img_h) = (img.width(), img.height() / 2);
		let mut ww = width as u32;
		let mut hh = img_h * width as u32 / img_w;
		if hh > height as u32 {
			hh = height as u32;
			ww = img_w * height as u32 / img_h;
		}
		let col_img = img.resize_exact(ww, hh, image::imageops::FilterType::Triangle);
        for x in 0..ww {
            for y in 0..hh {
                let cell = col_img.get_pixel(x, y);
                let dep = rgb2dep(cell.0[0], cell.0[1], cell.0[2]);
                asc.push(
                    Node::new(
                        match color_flag {
                            0 => {
                                Color::Rgb([cell.0[0], cell.0[1], cell.0[2]])
                            },
                            1 => {
                                Color::My256(rgb2my256(cell.0[0], cell.0[1], cell.0[2]))
                            },
                            2 => {
                                Color::Df256(rgb2df256(cell.0[0], cell.0[1], cell.0[2]))
                            },
                            // to be add
                            _ => {
                                Color::Rgb([cell.0[0], cell.0[1], cell.0[2]])
                            }
                        },
                        dep/16, (x as u16, y as u16)
                    )
                );
            }
        }
        asc
    }

    // TODO fuck your CPU
    pub fn img2asc_threads(img: image::DynamicImage, width: u16, height: u16) -> Vec<Node> {
        let mut asc: Vec<Node> = Vec::new();
        asc
    }

    /// ori_fps -> tar_fps, not change when ori_fps < tar_fps
    /// return (fps, SHOULD_READ_FRAME_SET)
    pub fn rate_adjust(ori_fps: u16, tar_fps: u16) -> (u8, HashSet<u8>){
        if ori_fps <= tar_fps {
            let mut rt = HashSet::new();
            for idx in 0..ori_fps {
                rt.insert(idx as u8);
            }
            return (ori_fps as u8, rt);
        }
        let mut rt = HashSet::new();
        for idx in 0..ori_fps * tar_fps {
            if idx%ori_fps == 0 {
                rt.insert((idx / tar_fps) as u8);
            }
        }
        (tar_fps as u8, rt)
    }
}

use crate::data::Package;
use ffmpeg_next as ffmpeg;
use std::collections::HashSet;

pub fn process(input: &str, output: &str) {
    let pkg = ffmpeg_job(input);
    crate::data::pack(pkg, output);
}

fn ffmpeg_job(filename: &str) -> Package {
    let fps: u8;
    let (width, height) = termion::terminal_size().unwrap();
    let mut num_per_frame: Vec<u32> = Vec::new();
    let mut nodes: Vec<crate::player::Node> = Vec::new();

    // TODO
    ffmpeg::init().unwrap();
    let mut ictx = ffmpeg::format::input(&filename).unwrap();
    // audio
    if let Some(_audio) = ictx.streams().best(ffmpeg::media::Type::Audio) {
        let audio_index = _audio.index();
    }

    // video
    let _video = ictx.streams().best(ffmpeg::media::Type::Video)
        .unwrap();
    let video_index = _video.index();

    // get fps
    let fps_idx: HashSet<u8>;
    let ori_fps = |x: ffmpeg::Rational| -> u8 {
        if x.1 == 1 {
            return x.0 as u8;
        }
        else {
            return (x.0 / x.1) as u8;
        }
    }(_video.rate());
    (fps, fps_idx) = tools::rate_adjust(ori_fps as u16, 24);

    // TODO
    let context_decoder = ffmpeg::codec::context::Context::from_parameters(_video.parameters())
        .unwrap();
    let mut decoder = context_decoder.decoder()
        .video()
        .unwrap();
    let mut scaler = ffmpeg::software::scaling::Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            ffmpeg::format::Pixel::RGB24,
            decoder.width(),
            decoder.height(),
            ffmpeg::software::scaling::flag::Flags::BILINEAR,)
        .unwrap();

    Package::new(fps, width, height, num_per_frame, nodes)
}
