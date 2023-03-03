#![allow(unused_variables)]
mod splash_screen;
mod game;
mod grid;
mod main_menu;
mod mygui;
mod player;
mod game_over;
mod game_menu;

use sfmx::prelude::*;
use splash_screen::SplashScreenState;

pub struct GameData {
    pub clear_color: Color,
    pub screen_size: (u32, u32)
}

fn main() {

    let mut app_data = AppData::default();
    app_data.win_size = (600, 600);
    app_data.frame_rate = 30;
    app_data.win_style = Style::TITLEBAR | Style::CLOSE;

    let init_data = GameData { 
        clear_color: Color::rgb(245,245,250),
        screen_size: app_data.win_size
    };

    Application::build()
        .with_initial_state(SplashScreenState::new())
        .with_states_data(init_data)
        .build(app_data)
        .run();
}
