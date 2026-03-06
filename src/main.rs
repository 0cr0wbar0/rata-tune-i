use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::time::Duration;
use rodio::*;
use edar::*;
use crossterm::event;
use crossterm::event::*;
use ratatui::*;

fn main() -> Result<(), Box<dyn Error>> {
    play_track("media/smokey_the_bear.mp3")?;
    Ok(())
}


pub struct App {
    exit: bool,
    player: Player
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
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Left => self.next_track(),
            KeyCode::Right => self.replay_track().unwrap(),
            KeyCode::Up => self.volume_up(),
            KeyCode::Down => self.volume_down(),
            KeyCode::Enter => self.play_pause(),
            _ => {}
        }
    }

    fn next_track(&mut self) {
        self.player.skip_one();
    }

    fn replay_track(&mut self) -> Result<(), Box<dyn Error>> {
        self.player.try_seek(Duration::from_secs(0))?;
        Ok(())
    }

    fn volume_up(&mut self) {
        if self.player.volume() < 1.0 {
            self.player.set_volume(self.player.volume() + 0.1);
        }
    }

    fn volume_down(&mut self) {
        if self.player.volume() > 0.0 {
            self.player.set_volume(self.player.volume() - 0.1);
        }
    }

    fn play_pause(&mut self) {
        if self.player.is_paused() {
            self.player.play();
        } else {
            self.player.pause();
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
