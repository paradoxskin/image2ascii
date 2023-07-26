use std::collections::VecDeque;
use binrw::binrw;

#[binrw]
pub struct Node {
    node_col: crate::color::Color,
    node_style: u8,
    pos: (u16, u16),
}

impl Node {
	const CHAR: [char; 16] = [' ', '.', ':', ';', 'i', '7', 'r', 'a', 'Z', 'S', '0', '8', 'X', 'M', 'W', '@'];

    pub fn new(color: crate::color::Color, style: u8, pos: (u16, u16)) -> Self {
        Self {
            node_col: color,
            node_style: style,
            pos
        }
    }

    fn get_node_style(&self) -> char {
        Self::CHAR[self.node_style as usize]
    }
    fn get_node_col(&self) -> (u8, u8, u8) {
        crate::color::Color::get_rgb(&self.node_col)
    }
    fn get_pos(&self) -> (u16, u16) {
        self.pos
    }
}

pub struct NodeQue {
    queue: VecDeque<Vec<Node>>
}

impl NodeQue {
    pub fn new() -> Self {
        let queue: VecDeque<Vec<Node>> = VecDeque::new();
        Self {
            queue
        }
    }

    fn get_front(&mut self) -> Option<Vec<Node>> {
        self.queue.pop_front()
    }

    pub fn add_back(&mut self, a_frame: Vec<Node>) {
        self.queue.push_back(a_frame);
    }
}

struct PNode {
    node_col: (u8, u8, u8),
    style: char,
}

impl PNode {
    fn new(node_col: (u8, u8, u8), style: char) -> Self {
        Self {
            node_col,
            style
        }
    }

    fn same_with(&self, node: &Node) -> bool {
        self.style == node.get_node_style() && self.node_col == node.get_node_col()
    }

    fn change_to(&mut self, node: &Node) {
        self.node_col = node.get_node_col();
        self.style = node.get_node_style();
    }
}

struct Screen {
    width: u16,
    height: u16,
    screen: Vec<PNode>
}

impl Screen {
    fn new(width: u16, height: u16) -> Self {
        let mut screen: Vec<PNode> = Vec::new();
        for _ in 0..width {
            for _ in 0..height {
                screen.push(PNode::new((0, 0, 0), ' '));
            }
        }
        Self {
            width,
            height,
            screen,
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        x + y * self.width as usize
    }
}

pub struct Player {
    // music and images
    fps: u8,
    width: u16,
    height: u16,
    screen: Screen,
    node_que: NodeQue,
}

impl Player {
    pub fn new(fps: u8, width: u16, height: u16) -> Option<Self> {
        let (ww, hh) = termion::terminal_size().unwrap();
        if ww < width || hh < height {
            return None;
        }
        let screen = Screen::new(width, height);
        let mut node_que = NodeQue::new();
        // TODO add frames to node_que
        Some(Self {
            fps,
            width,
            height,
            screen,
            node_que
        })
    }

    pub fn mainloop(&mut self) {
        let wait = 1.0 / self.fps as f64;
        loop {
			let begin = std::time::Instant::now();
            let opt_frame = self.node_que.get_front();
            if let Some(frame) = opt_frame {
                for node in frame {
                    let pos = node.get_pos();
                    let idx = self.screen.get_index(pos.0 as usize, pos.1 as usize);
                    // TODO draw
                    self.screen.screen[idx].change_to(&node);
                }
                // TODO skip if slow
                let pass = std::time::Instant::now().duration_since(begin).as_secs_f64();
                let dis = wait - pass;
                if dis > 0. {
                    std::thread::sleep(std::time::Duration::from_secs_f64(wait - pass));
                }
            }
            else {
                break;
            }
        }
    }
}
