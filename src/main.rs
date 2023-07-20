#![allow(unused_imports)]
use anyhow::{anyhow, Result};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use rodio::source::{SineWave, Source};
use rodio::{Decoder, OutputStream, Sink};
use std::io::BufReader;
use std::{fs::File, io::BufRead};
use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Widget},
    Terminal,
};

fn play_sound() -> Result<()> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("./test.mp3").unwrap());
    let source = Decoder::new(file).unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.append(source);
    sink.set_volume(0.2);

    thread::sleep(Duration::from_millis(10000));

    sink.stop();

    Ok(())
}

fn main() -> Result<()> {
    start_ui()?;

    Ok(())
}

fn close_application() -> Result<()> {
    disable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}

fn start_ui() -> Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;

    // Do stuff
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default().title("Player").borders(Borders::ALL);
        f.render_widget(block, size);
    })?;

    play_sound()?;

    // Shut up shop
    close_application()?;
    Ok(())
}
