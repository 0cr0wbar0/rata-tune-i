use crossterm::event;
use crossterm::event::*;
use edar::*;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::prelude::Stylize;
use ratatui::symbols::border;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Paragraph, Widget};
use ratatui::*;
use rodio::*;
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::time::Duration;

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::new().run(terminal))
}

pub enum AppState {
    UNLOADED,
    PLAYING,
    PAUSED,
}

pub struct App {
    exit: bool,
    player: Player,
    current_song_name: &'static str,
    state: AppState,
}

impl App {

    pub fn new() -> Self {
        let (player, _) = Player::new();
        Self { exit: false, player, current_song_name: "null", state: AppState::UNLOADED }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
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
            KeyCode::Char('q') => self.exit = true,
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

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Rata-tune-i ".bold());
        match self.state {
            AppState::UNLOADED => {
                let instructions = Line::from(vec![
                    " Replay ".into(),
                    " <Left> ".red().bold(),
                    " Play/Pause ".into(),
                    " <Enter> ".red().bold(),
                    " Next ".into(),
                    " <Right> ".red().bold(),
                    " Exit ".into(),
                    " <q> ".red().bold(),
                ]);
                let block = Block::bordered()
                    .title(title.centered())
                    .title_bottom(instructions.centered())
                    .border_set(border::THICK);
                let player_text = Text::from(vec![Line::from(vec![
                    "NO FILE LOADED".into()
                ])]);

                Paragraph::new(player_text)
                    .centered()
                    .block(block)
                    .render(area, buf);
            }
            AppState::PAUSED => {
                let instructions = Line::from(vec![
                    " Replay ".into(),
                    " <Left> ".green().bold(),
                    " Play ".into(),
                    " <Enter> ".green().bold(),
                    " Next ".into(),
                    " <Right> ".green().bold(),
                    " Exit ".into(),
                    " <q> ".green().bold(),
                ]);
                let block = Block::bordered()
                    .title(title.centered())
                    .title_bottom(instructions.centered())
                    .border_set(border::THICK);
                let player_text = Text::from(vec![Line::from(vec![
                    "PAUSED: ".into(),
                    self.current_song_name.into(),
                ])]);

                Paragraph::new(player_text)
                    .centered()
                    .block(block)
                    .render(area, buf);
            }
            AppState::PLAYING => {
                let instructions = Line::from(vec![
                    " Replay ".into(),
                    " <Left> ".green().bold(),
                    " Pause ".into(),
                    " <Enter> ".green().bold(),
                    " Next ".into(),
                    " <Right> ".green().bold(),
                    " Exit ".into(),
                    " <q> ".green().bold(),
                ]);
                let block = Block::bordered()
                    .title(title.centered())
                    .title_bottom(instructions.centered())
                    .border_set(border::THICK);
                let player_text = Text::from(vec![Line::from(vec![
                    "Currently playing: ".into(),
                    self.current_song_name.into(),
                ])]);

                Paragraph::new(player_text)
                    .centered()
                    .block(block)
                    .render(area, buf);
            }
        }
    }
}


fn play_track(music_file: &str) -> Result<Player, Box<dyn Error>> {
    // Get an output stream handle to the default physical sound device.
    // Note that the playback stops when the stream_handle is dropped.//!
    let sink_handle = DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
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
