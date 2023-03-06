use sfmx::prelude::*;

pub mod main_state;
pub mod chip8;


fn main() {

    let mut app_data = AppData::default();
    let scale = 16;
    let chip8_screen_size = (64, 32);
    let offset = Vector2u::new(200, 200);
    app_data.win_size = (chip8_screen_size.0 * scale + offset.x, chip8_screen_size.1 * scale + offset.y);
    app_data.frame_rate = 0;
    app_data.enable_vsync = false;
    app_data.win_style = Style::CLOSE;

    let init_data = ();

    let init_state = main_state::MainState::new(scale as f32);


    Application::build()
        .with_initial_state(init_state)
        .with_states_data(init_data)
        .build(app_data)
        .run();
}
