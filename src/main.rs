extern "C" {
    fn D_DoomMain();
}

mod net;
mod sound;
mod video;

fn main() {
    unsafe {
        D_DoomMain();
    }
}
