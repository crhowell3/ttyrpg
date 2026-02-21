use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Style;
use ratatui::widgets::{Block, BorderType, Borders};
use std::rc::Rc;

#[derive(Clone)]
pub struct UI {}

impl Default for UI {
    fn default() -> Self {
        Self {}
    }
}

impl UI {
    fn inventory_render(frame: &mut Frame, layout: Rc<[Rect]>) {
        let inventory_block = Block::default()
            .title("Inventory")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ratatui::style::Color::Gray))
            .border_type(BorderType::Rounded);

        frame.render_widget(inventory_block.clone(), layout[0]);
    }

    fn hp_render(frame: &mut Frame, layout: Rc<[Rect]>) {
        let hp_block = Block::default()
            .title("HP")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ratatui::style::Color::Green))
            .border_type(BorderType::Rounded);

        frame.render_widget(hp_block.clone(), layout[1]);
    }

    pub fn character_info_render(&self, area: Rect, frame: &mut Frame) {
        let width = area.width;

        let top_panel_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Length(width / 2 + 5),
                    Constraint::Length(width / 2 - 5),
                ]
                .as_ref(),
            )
            .split(area);

        Self::inventory_render(frame, top_panel_layout.clone());
        Self::hp_render(frame, top_panel_layout.clone());
    }

    pub fn world_render(&self, area: Rect, frame: &mut Frame) {
        let world_panel_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1)].as_ref())
            .split(area);

        let world_block = Block::default()
            .title("World")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ratatui::style::Color::Blue))
            .border_type(BorderType::Rounded);

        frame.render_widget(world_block.clone(), world_panel_layout[0]);
    }

    pub fn dialog_render() {}
}
