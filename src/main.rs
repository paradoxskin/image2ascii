mod utils;

use utils::{Show, Read};
fn main() {
	let picts: Read = Read::init();
	let mut show: Show = Show::init();
	let image = Read::img_file("/home/paradoxd/Pictures/wallpp/cropped-1920-1080-1302087.jpg");
	let nodes = Read::read_from_img(image, show.get_size());
	picts.add_back(nodes);
	show.run(&picts);
}
