use binrw::binrw;
use crate::player::{Node, NodeQue};

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
    pub fn output(pkg: Package) -> crate::player::NodeQue {
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
