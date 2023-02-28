use sfmx::prelude::*;
use crate::{main_menu::MainMenuState, GameData};

#[cfg(debug_assertions)]
const MOVE_NEXT_STATE_TIME: f32 = 0.0;

#[cfg(not(debug_assertions))]
const MOVE_NEXT_STATE_TIME: f32 = 2.0;

pub struct SplashScreenState {
    title_sprite: RcSprite,
    timer: Timer
}

impl SplashScreenState {
    pub fn new() -> Self {
        Self {
            title_sprite: RcSprite::new(),
            timer: Timer::new()
        }
    }

    fn init_title_sprite(&mut self, title_texture: &RcTexture, win_size: (u32, u32)) {
        self.title_sprite.set_texture(title_texture, true);
        let title_size = Vector2f::new(320.0, 320.0);
        let texture_size = title_texture.size().as_other::<f32>();
        self.title_sprite.set_scale((
            title_size.x / texture_size.x,
            title_size.y / texture_size.y
        ));
        self.title_sprite.set_position((
            (win_size.0 as f32 - title_size.x) / 2.0,
            (win_size.1 as f32 - title_size.y) / 2.0,
        ));
    }
}

impl State<GameData> for SplashScreenState {
    
    fn on_init(&mut self, state_data: &mut StateData<GameData>) {
        //Game assets
        state_data.assets_manager.load_asset_buffered(AssetType::Texture, "X".to_string(), "assets/textures/icon_x.png");
        state_data.assets_manager.load_asset_buffered(AssetType::Texture, "O".to_string(), "assets/textures/icon_o.png");
        state_data.assets_manager.load_asset_buffered(AssetType::Texture, "white".to_string(), "assets/textures/white_texture.png");
        state_data.assets_manager.load_asset_buffered(AssetType::Texture, "start_btn".to_string(), "assets/textures/start_btn.png");
        state_data.assets_manager.load_asset_buffered(AssetType::Texture, "quit_btn".to_string(), "assets/textures/quit_btn.png");
        state_data.assets_manager.load_asset_buffered(AssetType::Texture, "restart_btn".to_string(), "assets/textures/restart_btn.png");
        state_data.assets_manager.load_asset_buffered(AssetType::Texture, "arrow".to_string(), "assets/textures/arrow.png");

        state_data.assets_manager.load_asset_buffered(AssetType::Font, "font".to_string(), "assets/fonts/Silkscreen/slkscre.ttf");
        ////////////

        // State assets
        state_data.assets_manager.load_asset(AssetType::Texture, "title".to_string(), "assets/textures/title.png");
        ////

        let title_texture = state_data.assets_manager.get_asset(AssetType::Texture, "title".to_string()).unwrap();
        let win_size = state_data.data.screen_size;
        self.init_title_sprite(title_texture, win_size);
        self.timer.restart();
    }

    fn on_update(&mut self, state_data: &mut StateData<GameData>) -> Transition<GameData> {

        state_data.assets_manager.launch_loadings();

        if self.timer.elapsed().as_secs_f32() > MOVE_NEXT_STATE_TIME  && state_data.assets_manager.loading_percentage() >= 100.0{
            return Transition::Replace(Box::new(MainMenuState::new()));
        }
        Transition::None
    }

    fn on_render(&mut self, state_data: &mut StateData<GameData>, window: &mut RenderWindow) {
        window.clear(state_data.data.clear_color);
        window.draw(&self.title_sprite);
        window.display();
    }
}

