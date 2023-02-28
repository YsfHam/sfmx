use sfmx::prelude::*;
use crate::game_over::GameOverState;
use crate::grid::{DrawableGrid, GameStatus};
use crate::GameData;
use crate::player::Player;
use crate::gui::{Button, ButtonsGroup};

enum GamePlayState {
    Playing,
    GameOverAnimation,
    GotoGameOverState
}

pub struct GameState {
    grid: Option<DrawableGrid>,
    players: [Box<dyn Player>; 2],
    current_player_index: usize,
    game_play_state: GamePlayState,
    game_status: GameStatus,
    game_over_timer: Timer,
    grid_data: GridData,
    buttons: ButtonsGroup

}

#[derive(Copy, Debug, Clone)]
pub struct GridData {
    pub grid_size: usize,
    pub sym_occs_win: u32
}

impl GameState {
    pub fn new(grid_data: GridData, players: [Box<dyn Player>; 2]) -> Self {
        GameState {
            grid: None,
            players,
            current_player_index: 0,
            game_play_state: GamePlayState::Playing,
            game_status: GameStatus::NotFinished,
            game_over_timer: Timer::new(),
            grid_data,
            buttons: ButtonsGroup::new()
        }
    }
}

impl State<GameData> for GameState {

    fn on_init(&mut self, state_data: &mut StateData<GameData>) {
        let x_texture = state_data.assets_manager.get_asset(AssetType::Texture, "X".to_string()).unwrap();
        let o_texture = state_data.assets_manager.get_asset(AssetType::Texture, "O".to_string()).unwrap();
        let white_texture = state_data.assets_manager.get_asset(AssetType::Texture, "white".to_string()).unwrap();
        let quit_btn_texture = state_data.assets_manager.get_asset(AssetType::Texture, "quit_btn".to_string()).unwrap();

        let mut dims = Vector2::from(state_data.data.screen_size).as_other();
        dims.y = dims.y * 0.9;
        let mut grid = DrawableGrid::new([white_texture, x_texture, o_texture],
            10.0, 
            self.grid_data.grid_size, 
            self.grid_data.sym_occs_win, 
            dims
        );
        grid.set_position(Vector2f::new(0.0, state_data.data.screen_size.1 as f32 - dims.y));
        self.grid = Some(grid);

        self.players[0].allow();
        let button_height = state_data.data.screen_size.1 as f32 - dims.y;
        let quit_btn = Button::new(quit_btn_texture, (button_height * 2.0, button_height));
        self.buttons.add_button("quit_btn", quit_btn);

    }

    fn on_event(&mut self, event: Event, state_data: &mut StateData<GameData>) -> Transition<GameData> {
        self.buttons.on_event(event);
        self.players[self.current_player_index].on_event(event);
        Transition::None
    }

    fn on_update(&mut self, state_data: &mut StateData<GameData>) -> Transition<GameData> {

        match self.game_play_state {
            GamePlayState::Playing => {
                let grid = self.grid.as_mut().unwrap();
                let current_player = &mut self.players[self.current_player_index];
                if current_player.play(grid) {
                    current_player.forbid();
                    self.game_status = grid.get_winner();
                    if self.game_status != GameStatus::NotFinished {
                        if self.game_status == GameStatus::Draw {
                            self.game_play_state = GamePlayState::GotoGameOverState;
                        }
                        else {
                            grid.init_winning_line();
                            self.game_over_timer.restart();
                            self.game_play_state = GamePlayState::GameOverAnimation;
                        }
                        for player in self.players.iter_mut() {
                            (*player).forbid();
                        }
                        return Transition::None;
                    }
                    else {
                        self.current_player_index = (self.current_player_index + 1) % 2;
                        self.players[self.current_player_index].allow();
                    }
                }

                if self.buttons.get_button("quit_btn").is_clicked() {
                    return Transition::Remove;
                }

                self.buttons.reset();

                Transition::None
            },
            GamePlayState::GameOverAnimation => {
                if self.game_over_timer.elapsed().as_secs_f32() >= 1.25 {
                    self.game_play_state = GamePlayState::GotoGameOverState;
                }
                Transition::None
            },
            GamePlayState::GotoGameOverState => {
                return Transition::Replace(Box::new(GameOverState::new(self.game_status)))
            }
        }
    }

    fn on_render(&mut self, state_data: &mut StateData<GameData>, window: &mut dyn RenderTarget) -> bool {
        if !self.grid.as_ref().unwrap().can_draw_sprites() {
            return false;
        }
        window.clear(state_data.data.clear_color);
        self.grid.as_mut().unwrap().draw(window);
        self.buttons.draw(window);

        return true;
    }
}