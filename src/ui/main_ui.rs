use ratatui::style::Style;
use ratatui::widgets::Block;
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame<'_>) {
    let main_area = frame.area();
    render_game_widgets(frame, app, main_area);
}

pub fn render_game_widgets(frame: &mut Frame<'_>, app: &mut App, main_area: Rect) {
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Ratio(1, 20),
                Constraint::Ratio(18, 20),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(main_area);

    let character_info_block = Block::default().style(Style::default());

    app.game
        .ui
        .character_info_render(character_info_block.inner(main_layout[0]), frame);

    let world_block = Block::default().style(Style::default());

    app.game.ui.world_render(world_block.inner(main_layout[1]), frame);
}
