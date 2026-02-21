use crate::app::{App, AppResult};
use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

pub fn handle_key_events(key_event: KeyEvent, app: &mut App) -> AppResult<()> {
    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    match key_event.code {
        KeyCode::Char('q') => app.quit(),
        _ => {}
    }
    Ok(())
}
