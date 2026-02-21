#[derive(Debug, Default)]
struct AppState {
    character: Character,
}

#[derive(Debug, Default)]
struct Items {
    name: String,
    description: String,
}

#[derive(Debug, Default)]
struct Character {
    health: u8,
    items: Vec<Items>,
}
