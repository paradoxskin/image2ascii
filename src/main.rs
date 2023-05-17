mod utils;

use utils::{Show, Read};
fn main() {
	let picts: Read = Read::init();
	let mut show: Show = Show::init();
	show.run(&picts);
}
