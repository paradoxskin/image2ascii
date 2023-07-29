use std::collections::VecDeque;
use binrw::binrw;
use crate::data;
use image::io::Reader;
use termion::raw::IntoRawMode;
use std::io::Write;

#[binrw]
pub struct Node {
    node_col: crate::color::Color,
    node_style: u8,
    pos: (u16, u16),
}

impl Node {
	const CHAR: [char; 16] = ['.', '.', ':', ';', 'i', '7', 'r', 'a', 'Z', 'S', '0', '8', 'X', 'M', 'W', '@'];

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
    pub fn new(fps: u8, width: u16, height: u16, s_width: u16, s_height: u16, node_que: NodeQue) -> Option<Self> {
        if s_width < width || s_height < height {
            return None;
        }
        let screen = Screen::new(width, height);
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
        print!("{}{}", termion::cursor::Hide, termion::clear::All);
        let mut stdout = std::io::stdout().into_raw_mode().unwrap();
        loop {
			let begin = std::time::Instant::now();
            let opt_frame = self.node_que.get_front();
            if let Some(frame) = opt_frame {
                for node in frame {
                    let pos = node.get_pos();
                    let idx = self.screen.get_index(pos.0 as usize, pos.1 as usize);
                    let rgb = node.get_node_col();
                    let style = node.get_node_style();
                    write!(stdout, "{}{}{}", termion::cursor::Goto(pos.0 + 1, pos.1 + 1), termion::color::Fg(termion::color::Rgb(rgb.0, rgb.1, rgb.2)), style).unwrap();
                    self.screen.screen[idx].change_to(&node);
                }
                stdout.flush().unwrap();
                // TODO- skip if slow
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
        print!("{}", termion::cursor::Show);
    }
}

pub fn play(filename: &str) {
    let (s_width, s_height) = termion::terminal_size().unwrap();
    let o_player: Option<Player>;
    match is_img(filename, s_width, s_height) {
        None => {
            let pkg = data::unpack(filename);
            let (fps, width, height) = pkg.get_config();
            o_player = Player::new(fps, width, height, s_width, s_height, data::Package::open(pkg));
        },
        Some(node_que) => {
            // no need to know ascii_img's width and height
            o_player = Player::new(1, s_width, s_height, s_width, s_height, node_que);
        }
    }
    if let Some(mut player) = o_player {
        // play music?
        player.mainloop();
        return;
    }
    println!("screen too small ~\nrebuild the origin video\n\n{}", crate::HELP);
}

pub fn is_img(filename: &str, s_width: u16, s_height: u16) -> Option<NodeQue> {
    if let Ok(file) = Reader::open(filename) {
        if let Ok(img) = file.decode() {
            let mut node_que = NodeQue::new();
            // TODO color option
            let tmp = crate::reader::tools::img2asc(img, s_width, s_height, 2);
            node_que.add_back(tmp);
            return Some(node_que);
        }
    }
    None
}
