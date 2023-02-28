
use sfmx::prelude::*;
use crate::{GameData, grid::GameStatus};
use crate::gui::*;

pub struct GameOverState {
    game_status: GameStatus,
    text: String,
    buttons: ButtonsGroup
}

impl GameOverState {
    pub fn new(game_status: GameStatus) -> GameOverState {
        Self {
            game_status,
            text: String::from("Game Over\n\n"),
            buttons: ButtonsGroup::new()
        }
    }
}

impl State<GameData> for GameOverState {
    fn on_init(&mut self, state_data: &mut StateData<GameData>) {

        match self.game_status {
            GameStatus::Draw => self.text.push_str("Draw"),
            GameStatus::Winner(s) => self.text.push_str(&format!("Winner is {:?}", s)),
            _ => {}
        }

        let restart_btn_texture = state_data.assets_manager.get_asset(AssetType::Texture, "restart_btn".to_string()).unwrap();
        let quit_btn_texture = state_data.assets_manager.get_asset(AssetType::Texture, "quit_btn".to_string()).unwrap();

        let buttons_size = Vector2f::new(200.0, 100.0);
        let win_size = Vector2::from(state_data.data.screen_size).as_other::<f32>();

        let mut restart_btn  = Button::new(restart_btn_texture, buttons_size);
        restart_btn.set_position((
            (win_size.x - buttons_size.x) / 2.0,
            win_size.y * 2.0 / 3.0 - buttons_size.y + 30.0
        ));
        self.buttons.add_button("restart", restart_btn);

        let mut qui_btn = Button::new(quit_btn_texture, buttons_size);
        qui_btn.set_position((
            (win_size.x - buttons_size.x) / 2.0,
            win_size.y * 2.0 / 3.0 + 50.0
        ));
        self.buttons.add_button("quit", qui_btn);
    }


    fn on_event(&mut self, event: Event, state_data: &mut StateData<GameData>) -> Transition<GameData> {
        self.buttons.on_event(event);
        Transition::None
    }

    fn on_update(&mut self, state_data: &mut StateData<GameData>) -> Transition<GameData> {
        if self.buttons.get_button("restart").is_clicked() {
            return Transition::Remove;
        }

        if self.buttons.get_button("quit").is_clicked() {
            return Transition::Quit;
        }

        Transition::None
    }

    fn on_render(&mut self, state_data: &mut StateData<GameData>, window: &mut RenderWindow) {
        window.clear(state_data.data.clear_color);
        let font: &SfBox<Font> = state_data.assets_manager.get_asset(AssetType::Font, "font".to_string()).unwrap();
        let mut text_ui = Text::new(&self.text, font, 50);
        text_ui.set_fill_color(Color::BLACK);
        text_ui.set_position((
            (window.size().x as f32 - text_ui.global_bounds().width as f32) / 2.0,
            window.size().y as f32 / 3.0 - text_ui.global_bounds().height as f32 / 2.0
        ));
        window.draw(&text_ui);

        self.buttons.draw(window);

        window.display();
    }
}