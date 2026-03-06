use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufReader;
use rodio::*;
use edar::*;
use crossterm::event;
use crossterm::event::*;
use ratatui::*;

fn main() -> io::Result<()> {
    ratatui::run(|terminal: &mut DefaultTerminal| {
        loop {
            terminal.draw(|frame| frame.render_widget("Hello World!", frame.area()))?;
            if event::read()?.is_key_press() {
                break Ok(());
            }
        }
    })
}

#[derive(Debug, Default)]
pub struct App {
    exit: bool,

}

impl App {

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        todo!()
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Left => {}
            KeyCode::Right => {}
            KeyCode::Up => {}
            KeyCode::Down => {}
            KeyCode::Enter => {}
            _ => {}
        }
    }



}

fn play_track(music_file: &str) -> Result<Player, Box<dyn Error>> {
    // Get an output stream handle to the default physical sound device.
    // Note that the playback stops when the stream_handle is dropped.//!
    let sink_handle = DeviceSinkBuilder::open_default_sink()
        .expect("open default audio stream");
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = BufReader::new(File::open(&music_file)?);
    let duration = extract_duration(&music_file).unwrap();
    // Decode that sound file into a source
    let player = rodio::play(&sink_handle.mixer(), file)?;

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(duration);

    Ok(player)
}
