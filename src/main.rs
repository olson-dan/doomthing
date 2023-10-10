extern "C" {
    fn D_DoomMain();
}

mod net;
mod sound;
mod video;
mod wad;

fn main() {
    unsafe {
        D_DoomMain();
    }
}
