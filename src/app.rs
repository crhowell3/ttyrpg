use crate::game_logic::game::Game;

pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct App {
    pub running: bool,
    pub game: Game,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            game: Game::default(),
        }
    }
}

impl App {
    pub fn tick(&mut self) {
        // some game logic
    }

    pub fn quit(&mut self) {
        self.running = false;
    }
}
