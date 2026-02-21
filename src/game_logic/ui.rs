use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
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
            .title(" INV ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ratatui::style::Color::Gray))
            .border_type(BorderType::Rounded);

        frame.render_widget(inventory_block.clone(), layout[0]);
    }

    fn hp_render(frame: &mut Frame, layout: Rc<[Rect]>) {
        let hp_block = Block::default()
            .title(" HP ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ratatui::style::Color::Green))
            .border_type(BorderType::Rounded);

        let hp_hearts = Paragraph::new(" 3 / 5 ")
            .alignment(Alignment::Center)
            .add_modifier(Modifier::BOLD);

        frame.render_widget(hp_block.clone(), layout[1]);
        frame.render_widget(hp_hearts, hp_block.inner(layout[1]));
    }

    fn mana_render(frame: &mut Frame, layout: Rc<[Rect]>) {
        let mana_block = Block::default()
            .title(" MANA ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ratatui::style::Color::LightBlue))
            .border_type(BorderType::Rounded);

        let mana_pips = Paragraph::new(" 5 / 5 ")
            .alignment(Alignment::Center)
            .add_modifier(Modifier::BOLD);

        frame.render_widget(mana_block.clone(), layout[2]);
        frame.render_widget(mana_pips, mana_block.inner(layout[2]));
    }

    pub fn character_info_render(&self, area: Rect, frame: &mut Frame) {
        let top_panel_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Ratio(1, 2),
                    Constraint::Ratio(1, 4),
                    Constraint::Ratio(1, 4),
                ]
                .as_ref(),
            )
            .split(area);

        Self::inventory_render(frame, top_panel_layout.clone());
        Self::hp_render(frame, top_panel_layout.clone());
        Self::mana_render(frame, top_panel_layout.clone());
    }

    pub fn world_render(&self, area: Rect, frame: &mut Frame) {
        let world_panel_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Fill(1)].as_ref())
            .split(area);

        let world_block = Block::default()
            .title(" WORLD ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(ratatui::style::Color::White))
            .border_type(BorderType::Rounded);

        let world_components = Paragraph::new(" ♠️ ").alignment(Alignment::Center);

        frame.render_widget(world_block.clone(), world_panel_layout[0]);
        frame.render_widget(world_components, world_block.inner(world_panel_layout[0]));
    }

    pub fn dialog_render() {}
}
