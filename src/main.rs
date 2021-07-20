use sfml::{
    audio::Music,
    system::InputStream,
};
use std::io::Cursor;

fn load_120_bpm() -> Cursor<Vec<u8>> {
    let wav = std::fs::read("120_bpm.wav").unwrap();
    let stream = Cursor::new(wav);
    stream
}

fn main() {
    let mut mc = load_120_bpm();
    let mut ist = InputStream::new(&mut mc);
    let mut music =
        Music::from_stream(&mut ist)
        .expect("failed to convert 120_bpm.wav");
    music.play();
    drop(music); // After that, the program terminates with exit_code 0xC0000409 and STATUS_STACK_BUFFER_OVERRUN.
    println!("All good!");
}
