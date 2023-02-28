use sfmx::prelude::*;
use crate::GameData;
use crate::game::{GameState, GridData};
use crate::gui::{Button, ButtonsGroup};
use crate::player::{PlayerType, DumpPlayer, HumanPlayer, Player};
pub struct GameMenuState {
    grid_data: GridData,
    sym_occ_win_range: (u32, u32),
    buttons_group: ButtonsGroup,
    player_types: [PlayerType; 2],
    player_choice_bounds: [FloatRect; 2]
}

impl GameMenuState {
    pub fn new() -> GameMenuState {
        Self {
            grid_data: GridData {
                grid_size: 3,
                sym_occs_win: 3
            },
            sym_occ_win_range: (3, 7),
            buttons_group: ButtonsGroup::new(),
            player_types: [PlayerType::Human, PlayerType::Human],
            player_choice_bounds: [FloatRect::default(); 2]

        }
    }
}

impl State<GameData> for GameMenuState {

    fn on_init(&mut self, state_data: &mut StateData<GameData>) {
        let arrow_texture = state_data.assets_manager.get_asset(AssetType::Texture, "arrow".to_string()).unwrap();
        let win_size = Vector2::from(state_data.data.screen_size).as_other::<f32>();
        let window_eigth = win_size.x / 8.0;
        let buttons_height = 150.0;
        let buttons_offset = 50.0;
        let buttons_size = Vector2f::new(100.0, 100.0);
        
        let mut arrow_right_btn1 = Button::new(
            arrow_texture,
            buttons_size
        );
        arrow_right_btn1.set_position((
            window_eigth + buttons_size.x / 2.0 + buttons_offset,
            win_size.y - (buttons_height + buttons_size.y)
        ));

        self.buttons_group.add_button("right_arrow1", arrow_right_btn1);

        let mut arrow_left_btn1 = Button::new(
            arrow_texture,
            buttons_size
        );
        arrow_left_btn1.set_rotation(180.0);
        arrow_left_btn1.set_position((
            window_eigth + buttons_size.x / 2.0,
            win_size.y - buttons_height
        ));
        self.buttons_group.add_button("left_arrow1", arrow_left_btn1);
        

        //
        let mut arrow_right_btn2 = Button::new(
            arrow_texture,
            buttons_size
        );

        arrow_right_btn2.set_position((
            5.0 * window_eigth + buttons_size.x / 2.0 + buttons_offset / 2.0 + buttons_offset,
            win_size.y - (buttons_height + buttons_size.y)
        ));

        self.buttons_group.add_button("right_arrow2", arrow_right_btn2);

        let mut arrow_left_btn2 = Button::new(
            arrow_texture,
            buttons_size
        );
        arrow_left_btn2.set_rotation(180.0);
        arrow_left_btn2.set_position((
            5.0 * window_eigth + buttons_size.x / 2.0 + buttons_offset / 2.0,
            win_size.y - buttons_height
        ));
        self.buttons_group.add_button("left_arrow2", arrow_left_btn2);


        let start_btn_texture = state_data.assets_manager.get_asset(AssetType::Texture, "start_btn".to_string()).unwrap();
        let start_btn_size = Vector2f::from((200.0, 100.0));
        let mut start_btn = Button::new(start_btn_texture, start_btn_size);

        start_btn.set_position((
            (win_size.x - start_btn_size.x) / 2.0,
            win_size.y - start_btn_size.y * 1.25
        ));

        self.buttons_group.add_button("start_btn", start_btn);

    }

    fn on_resume(&mut self, state_data: &mut StateData<GameData>) {
        self.buttons_group.reset();
    }

    fn on_event(&mut self, event: Event, state_data: &mut StateData<GameData>) -> Transition<GameData> {
        self.buttons_group.on_event(event);
        if let Event::MouseButtonReleased { button, x, y } = event {
            for i in 0..2 {
                if self.player_choice_bounds[i].contains2(x as f32, y as f32) {
                    self.player_types[i] = match self.player_types[i] {
                        PlayerType::Human => PlayerType::AI,
                        PlayerType::AI => PlayerType::Human,
                    }
                }
            }
        }
        Transition::None
    }

    fn on_update(&mut self, state_data: &mut StateData<GameData>) -> Transition<GameData> {
        if self.buttons_group.get_button("left_arrow1").is_clicked() {
            self.grid_data.grid_size -= 1;
        }
        if self.buttons_group.get_button("right_arrow1").is_clicked() {
            self.grid_data.grid_size += 1;
        }
        self.grid_data.grid_size = self.grid_data.grid_size.clamp(
            self.sym_occ_win_range.0 as usize,
            self.sym_occ_win_range.1 as usize
        );

        if self.buttons_group.get_button("left_arrow2").is_clicked() {
            self.grid_data.sym_occs_win -= 1;
        }
        if self.buttons_group.get_button("right_arrow2").is_clicked() {
            self.grid_data.sym_occs_win += 1;
        }

        self.grid_data.sym_occs_win = self.grid_data.sym_occs_win.clamp(
            self.sym_occ_win_range.0,
            self.grid_data.grid_size as u32
        );

        if self.buttons_group.get_button("start_btn").is_clicked() {
            let mut players = self.player_types.iter().map::<Box<dyn Player>, _>(|v| match v {
                PlayerType::AI => Box::new(DumpPlayer::new()),
                PlayerType::Human => Box::new(HumanPlayer::new()),
            });
            let players = [players.next().unwrap(), players.next().unwrap()];
            return Transition::Add(Box::new(GameState::new(self.grid_data, players)));
        }

        self.buttons_group.reset();

        Transition::None
    }

    fn on_render(&mut self, state_data: &mut StateData<GameData>, window: &mut RenderWindow) {
        window.clear(state_data.data.clear_color);
        let font: &SfBox<Font> = state_data.assets_manager.get_asset(AssetType::Font, "font".to_string()).unwrap();
        let mut text = Text::default();
        text.set_font(font);
        text.set_fill_color(Color::BLACK);

        let window_quarter = window.size().x as f32 / 4.0;
        {
            text.set_character_size(50);
            text.set_string(&format!("{}", self.player_types[0]));
            text.set_position((window_quarter - text.global_bounds().width / 2.0, 10.0));
            self.player_choice_bounds[0] = text.global_bounds();
    
            window.draw(&text);
            
            text.set_string(&format!("{}", self.player_types[1]));
            text.set_position((-text.global_bounds().width / 2.0 + 3.0 * window_quarter, 10.0));
            self.player_choice_bounds[1] = text.global_bounds();
            window.draw(&text);

            text.set_string("X");
            text.set_position((window_quarter - text.global_bounds().width, 80.0));
    
            window.draw(&text);
            
            text.set_string("Y");
            text.set_position((window_quarter - text.global_bounds().width + 2.0 * window_quarter, 80.0));
            window.draw(&text);
        }

        let text_char_size = 100u32;
        let texts = [
            format!("{}x{}", self.grid_data.grid_size, self.grid_data.grid_size),
            format!("{}", self.grid_data.sym_occs_win)
        ];
        for (i, text_string) in texts.iter().enumerate() {
            text.set_string(text_string);
            text.set_character_size(text_char_size);
            
            let text_pos_x = (2.0 * i as f32 + 1.0) * window_quarter - text.global_bounds().width / 2.0;
            let text_pos_y = window.size().y as f32 / 2.0 - text_char_size as f32;
            text.set_position((text_pos_x, text_pos_y));
            window.draw(&text);
        }

        let texts = [
            "Grid\ndimensions",
            "Matching\npattern"
        ];
        let offset = 50.0;
        for (i, text_string) in texts.iter().enumerate() {
            text.set_string(*text_string);
            text.set_character_size(30);
            
            let text_pos_x = (2.0 * i as f32 + 1.0) * window_quarter - text.global_bounds().width / 2.0;
            let text_pos_y = window.size().y as f32 / 2.0 - text_char_size as f32 - offset;
            text.set_position((text_pos_x, text_pos_y));
            window.draw(&text);
        }

        self.buttons_group.draw(window);
        window.display();
    }
}