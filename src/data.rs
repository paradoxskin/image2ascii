use binrw::{binrw, BinRead, BinWrite};
use crate::player::{Node, NodeQue};
use std::io::{Cursor, Read, Write, BufReader};

#[binrw]
#[brw(big, magic = b"I2A")]
pub struct Package {
    fps: u8,
    width: u16,
    height: u16,
    num_per_frame_count: u32,
    #[br(count = num_per_frame_count)]
    num_per_frame: Vec<u32>,
    nodes_count: u64,
    #[br(count = nodes_count)]
    nodes: Vec<Node>
}

impl Package {
    pub fn new(fps: u8, width: u16, height: u16, num_per_frame: Vec<u32>, nodes: Vec<Node>) -> Self {
        Self {
            fps,
            width,
            height,
            num_per_frame_count: num_per_frame.len() as u32,
            num_per_frame,
            nodes_count: nodes.len() as u64,
            nodes
        }
    }
    /// -> (fps, width, height)
    pub fn get_config(&self) -> (u8, u16, u16) {
        (self.fps, self.width, self.height)
    }

    /// -> NodeQue
    pub fn open(pkg: Package) -> crate::player::NodeQue {
        let mut que = NodeQue::new();
        let mut iter = pkg.nodes.into_iter();
        for lmt in pkg.num_per_frame {
            let mut tmp_vec: Vec<Node> = Vec::new();
            for _ in 0..lmt {
                if let Some(node) = iter.next() {
                    tmp_vec.push(node);
                }
            }
            que.add_back(tmp_vec);
        }
        que
    }
}

/// data -> file
pub fn pack(package: Package, output_path: &str) {
    let mut buf = Cursor::new(Vec::<u8>::new());
    package.write(&mut buf).unwrap();
    buf.set_position(0);

    let mut encoder = flate2::bufread::DeflateEncoder::new(buf, flate2::Compression::best());
    let mut encoded_buf = Vec::new();
    encoder.read_to_end(&mut encoded_buf).unwrap();

    let mut file = std::fs::File::create(output_path).unwrap();
    file.write(encoded_buf.as_slice()).unwrap();
}

/// file -> data
pub fn unpack(file_path: &str) -> Package {
    let file = std::fs::File::open(file_path).unwrap();
    let buf = BufReader::new(file);

    let mut decoder = flate2::bufread::DeflateDecoder::new(buf);
    let mut decoded_buf = Vec::new();
    decoder.read_to_end(&mut decoded_buf).unwrap();

    Package::read(&mut Cursor::new(decoded_buf)).unwrap()
}
