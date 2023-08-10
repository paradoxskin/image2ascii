pub mod tools {
    use crate::player::Node;
    use crate::color::Color;
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

    pub fn get_tran_size(img_w: u32, img_h: u32, width: u16, height: u16) -> (u32, u32) {
        let img_h = img_h / 2;
        let mut ww = width as u32;
        let mut hh = img_h * width as u32 / img_w;
		if hh > height as u32 {
			hh = height as u32;
			ww = img_w * height as u32 / img_h;
		}
        (ww, hh)
    }

    /// color Option:
    /// 0 -> RGB
    /// 1 -> My256
    /// 2 -> Df256
    pub fn img2asc(frame: Vec<u8>, color_flag: u8) -> Vec<Node> {
        let mut asc: Vec<Node> = Vec::new();
        let len = frame.len() / 3;
        for idx in 0..len {
            let dep = rgb2dep(frame[idx * 3], frame[idx * 3 + 1], frame[idx * 3 + 2]);
                asc.push(
                    Node::new(
                        match color_flag {
                            0 => {
                                Color::Rgb([frame[idx * 3], frame[idx * 3 + 1], frame[idx * 3 + 2]])
                            },
                            1 => {
                                Color::My256(rgb2my256(frame[idx * 3], frame[idx * 3 + 1], frame[idx * 3 + 2]))
                            },
                            2 => {
                                Color::Df256(rgb2df256(frame[idx * 3], frame[idx * 3 + 1], frame[idx * 3 + 2]))
                            },
                            // to be add
                            _ => {
                                Color::Rgb([frame[idx * 3], frame[idx * 3 + 1], frame[idx * 3 + 2]])
                            }
                        },
                        dep/16, (0 as u16, 0 as u16)
                    )
                );
        }
        asc
    }

    // TODO fuck your CPU
    //pub fn img2asc_threads(frame: Vec<u8>, color_flag: u8) -> Vec<Node> {
    //    let asc: Vec<Node> = Vec::new();
    //    asc
    //}

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
    let pkg = ffmpeg_job(input, output);
    crate::data::pack(pkg, output);
}

fn ffmpeg_job(filename: &str, output_path: &str) -> Package {
    let fps: u8;
    let (width, height) = termion::terminal_size().unwrap();
    let mut num_per_frame: Vec<u32> = Vec::new();
    let mut nodes: Vec<crate::player::Node> = Vec::new();

    ffmpeg::init().unwrap();
    let mut ictx = ffmpeg::format::input(&filename).unwrap();
    // audio TODO
    if let Some(_audio) = ictx.streams().best(ffmpeg::media::Type::Audio) {
        let audio_index = _audio.index();
        let output_path = format!("{}.wav", output_path);
        save_audio(&output_path);
    }

    // video
    let _video = ictx.streams().best(ffmpeg::media::Type::Video)
        .unwrap();
    let video_index = _video.index();
    let fps_idx: HashSet<u8>;
    let context_decoder = ffmpeg::codec::context::Context::from_parameters(_video.parameters())
        .unwrap();
    let mut decoder = context_decoder.decoder()
        .video()
        .unwrap();
    let (img_w, img_h) = (decoder.width(), decoder.height());
    let (tran_w, tran_h) = tools::get_tran_size(img_w, img_h, width, height);
    let mut scaler = ffmpeg::software::scaling::Context::get(
            decoder.format(),
            img_w,
            img_h,
            ffmpeg::format::Pixel::RGB24,
            tran_w,
            tran_h,
            ffmpeg::software::scaling::flag::Flags::BILINEAR).unwrap();
    // get fps
    let rate = _video.rate();
    let ori_fps = if rate.1 == 1 {rate.0 as u8} else {(rate.0 / rate.1) as u8};
    (fps, fps_idx) = tools::rate_adjust(ori_fps as u16, 24);

    // start trans images to ascii
    let mut count = 0u8;
    let mut last = Option::<Vec<crate::player::Node>>::None;
    for (stream, mut packet) in ictx.packets() {
        if stream.index() == video_index {
            decoder.send_packet(&mut packet).unwrap();
            let mut decoded = ffmpeg::util::frame::Video::empty();
            while decoder.receive_frame(&mut decoded).is_ok() {
                if let Some(_) = fps_idx.get(&count) {
                    let mut trans_frame = ffmpeg::util::frame::Video::empty();
                    scaler.run(&mut decoded, &mut trans_frame).unwrap();
                    let trans_data = Vec::from(trans_frame.data(0));
                    // color mod
                    let now = tools::img2asc(trans_data, 1);
                    let all_len = now.len() / tran_h as usize;
                    let mut p_count = 0;
                    if let Some(last_frame) = &mut last {
                        for x in 0..tran_w as usize {
                            for y in 0..tran_h as usize {
                                let idx = y * all_len + x;
                                let mut now_node = now[idx].clone();
                                if now_node != last_frame[idx] {
                                    now_node.set_xy(x as u16, y as u16);
                                    nodes.push(now_node);
                                    p_count += 1;
                                }
                            }
                        }
                    }
                    else { // first frame
                        for x in 0..tran_w as usize {
                            for y in 0..tran_h as usize {
                                let idx = y * all_len + x;
                                let mut now_node = now[idx].clone();
                                now_node.set_xy(x as u16, y as u16);
                                nodes.push(now_node);
                                p_count += 1;
                            }
                        }
                    }
                    num_per_frame.push(p_count);
                    last = Some(now);
                }
                count += 1;
                count %= fps;
            }
        }
    }

    Package::new(fps, width, height, num_per_frame, nodes)
}

fn save_audio(output_dir: &str) {
    // TODO
}

pub fn is_img(filename: &str, s_width: u16, s_height: u16) -> Option<crate::player::NodeQue> {
    ffmpeg::init().unwrap();
    if let Ok(mut ictx) = ffmpeg::format::input(&filename) {
        let mut node_que = crate::player::NodeQue::new();

        let _video = ictx.streams().best(ffmpeg::media::Type::Video)
            .unwrap();
        let video_index = _video.index();
        let context_decoder = ffmpeg::codec::context::Context::from_parameters(_video.parameters())
            .unwrap();
        let mut decoder = context_decoder.decoder()
            .video()
            .unwrap();
        let (img_w, img_h) = (decoder.width(), decoder.height());
        let (tran_w, tran_h) = tools::get_tran_size(img_w, img_h, s_width, s_height);
        let mut scaler = ffmpeg::software::scaling::Context::get(
                decoder.format(),
                img_w,
                img_h,
                ffmpeg::format::Pixel::RGB24,
                tran_w,
                tran_h,
                ffmpeg::software::scaling::flag::Flags::BILINEAR).unwrap();
        for (stream, mut packet) in ictx.packets() {
            if stream.index() == video_index {
                decoder.send_packet(&mut packet).unwrap();
                let mut decoded = ffmpeg::util::frame::Video::empty();
                while decoder.receive_frame(&mut decoded).is_ok() {
                    let mut nodes = Vec::<crate::player::Node>::new();
                    let mut trans_frame = ffmpeg::util::frame::Video::empty();
                    scaler.run(&mut decoded, &mut trans_frame).unwrap();
                    let trans_data = Vec::from(trans_frame.data(0));
                    // color mode
                    let now = tools::img2asc(trans_data, 1);
                    let all_len = now.len() / tran_h as usize;
                    for x in 0..tran_w as usize {
                        for y in 0..tran_h as usize {
                            let idx = y * all_len as usize + x;
                            let mut now_node = now[idx].clone();
                            now_node.set_xy(x as u16, y as u16);
                            nodes.push(now_node);
                        }
                    }
                    node_que.add_back(nodes);
                }
            }
        }
        return Some(node_que);
    }
    None
}
