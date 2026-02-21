pub type AppResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct App {
    pub running: bool,
}

impl Default for App {
    fn default() -> Self {
        Self { running: true }
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
