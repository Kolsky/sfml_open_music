use hound::{WavReader, WavWriter};
use sfml::{
    audio::Music,
    graphics::{Color, RenderTarget, RenderWindow},
    system::Time,
    window::{Event, Key, Style},
};
use std::io::Cursor;

fn load_120_bpm(times: usize) -> Cursor<Vec<u8>> {
    use std::fs::File;

    let file = File::open("120_bpm.wav").expect("failed to open 120_bpm.wav");
    let wav_reader = WavReader::new(file).expect("wav file isn't recognised");
    let spec = wav_reader.spec();
    let samples: hound::Result<_> = wav_reader.into_samples().collect();
    let samples: Vec<i16> = samples.expect("samples are not i16");
    let mut cursor = Cursor::new(vec![]);
    {
        let mut wav_writer = WavWriter::new(&mut cursor, spec).expect("oom");
        let mut sample_writer = wav_writer.get_i16_writer((samples.len() * times) as u32);
        std::iter::repeat(samples.as_slice())
            .take(times)
            .flatten()
            .for_each(|&sample| sample_writer.write_sample(sample));
        sample_writer.flush().unwrap();
    }
    cursor.set_position(0);
    cursor
}

fn load_120_bpm_v(times: usize) -> Vec<u8> {
    load_120_bpm(times).into_inner()
}

fn main() {
    let mut window =
        RenderWindow::new((800, 600), "SFML Window", Style::CLOSE, &Default::default());
    window.set_vertical_sync_enabled(true);

    #[allow(unused)]
    enum Load {
        FromFile,
        FromMemory,
        FromStream,
    }
    let load = Load::FromStream;

    let mut mc;
    let md;
    let mut music = match load {
        Load::FromFile => Music::from_file("120_bpm.wav"),
        Load::FromMemory => {
            md = load_120_bpm_v(8);
            Music::from_memory(&md)
        }
        Load::FromStream => {
            mc = load_120_bpm(8);
            Music::from_stream(&mut mc)
        }
    }
    .expect("failed to convert 120_bpm.wav");

    music.play();

    'running: loop {
        while let Some(event) = window.poll_event() {
            const STEP: Time = Time::milliseconds(0_250);
            match event {
                Event::Closed => break 'running,
                Event::KeyPressed {
                    code: Key::LEFT, ..
                } => {
                    let offset = music.playing_offset();
                    let offset = offset.max(STEP) - STEP;
                    music.play();
                    music.set_playing_offset(offset);
                }
                Event::KeyPressed {
                    code: Key::RIGHT, ..
                } => {
                    let offset = music.playing_offset() + STEP;
                    let offset = offset.min(music.duration());
                    music.set_playing_offset(offset);
                }
                _ => {}
            }
        }
        window.clear(Color::rgb(0x7f, 0xfd, 0xd4));
        window.display();
    }
}
