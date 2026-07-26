use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind};
use std::io;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppEvent {
    Key(KeyCode, KeyModifiers),
    ScrollUp,
    ScrollDown,
    Resize,
    Tick,
}

pub struct EventHandler;

impl EventHandler {
    pub fn next() -> io::Result<AppEvent> {
        if event::poll(Duration::from_millis(16))? {
            match event::read()? {
                Event::Key(key) if key.kind == KeyEventKind::Press => {
                    Ok(AppEvent::Key(key.code, key.modifiers))
                }
                Event::Mouse(m) => match m.kind {
                    MouseEventKind::ScrollDown => Ok(AppEvent::ScrollDown),
                    MouseEventKind::ScrollUp => Ok(AppEvent::ScrollUp),
                    _ => Ok(AppEvent::Tick),
                },
                Event::Resize(_, _) => Ok(AppEvent::Resize),
                _ => Ok(AppEvent::Tick),
            }
        } else {
            Ok(AppEvent::Tick)
        }
    }
}
