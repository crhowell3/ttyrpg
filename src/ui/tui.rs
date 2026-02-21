use crate::app::{App, AppResult};
use crate::event::EventHandler;
use crate::ui::main_ui;
use ratatui::Terminal;
use ratatui::backend::Backend;

#[derive(Debug)]
pub struct Tui<B: Backend> {
    terminal: Terminal<B>,
    pub events: EventHandler,
}

impl<B: Backend> Tui<B> {
    pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    pub fn draw(&mut self, app: &mut App) -> AppResult<()>
    where
        <B as Backend>::Error: 'static,
    {
        self.terminal.draw(|frame| main_ui::render(app, frame))?;
        Ok(())
    }
}
