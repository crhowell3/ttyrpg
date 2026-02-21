use super::ui::UI;

#[derive(Debug, Default, Clone)]
#[allow(dead_code)]
pub struct Items {
    name: String,
    description: String,
}

#[derive(Debug, Default)]
pub struct Character {
    health: u8,
    items: Vec<Items>,
}

impl Clone for Character {
    fn clone(&self) -> Self {
        let mut cloned_items: Vec<Items> = vec![];

        for i in 0..self.items.len() {
            cloned_items[i] = self.items[i].clone();
        }

        Self {
            health: self.health.clone(),
            items: cloned_items,
        }
    }
}

pub struct GameLogic {
    pub player: Character,
}

impl Clone for GameLogic {
    fn clone(&self) -> Self {
        GameLogic {
            player: self.player.clone(),
        }
    }
}

impl Default for GameLogic {
    fn default() -> Self {
        Self {
            player: Character::default(),
        }
    }
}

#[derive(Default)]
pub struct Game {
    pub logic: GameLogic,
    pub ui: UI,
}

impl Clone for Game {
    fn clone(&self) -> Self {
        Game {
            logic: self.logic.clone(),
            ui: self.ui.clone(),
        }
    }
}

impl Game {
    pub fn new() -> Self {
        Self {
            logic: GameLogic::default(),
            ui: UI::default(),
        }
    }
}

impl GameLogic {}
