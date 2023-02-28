use sfmx::prelude::*;

use crate::GameData;
use crate::game_menu::GameMenuState;
use crate::gui::{Button, ButtonsGroup};

struct MoveToAnimation {
    initial_position: Vector2f,
    target: Vector2f,
    lerp_acc: f32
}

impl MoveToAnimation {
    fn new(initial_position: Vector2f, target: Vector2f) -> Self{
        Self {
            initial_position,
            target,
            lerp_acc: 0.0
        }
    }

    fn update(&mut self, delta: f32) -> Vector2f {
        self.lerp_acc += delta;
        self.lerp_acc = self.lerp_acc.clamp(0.0, 1.0);
        (self.target - self.initial_position) * self.lerp_acc + self.initial_position
    }

    fn has_finished(&self) -> bool {
        self.lerp_acc >= 1.0
    }
}

pub struct MainMenuState {
    title_sprite: RcSprite,
    move_animation: Option<MoveToAnimation>,
    title_size: Vector2f,
    buttons: ButtonsGroup
}

impl MainMenuState {
    pub fn new() -> Self {
        Self {
            title_sprite: RcSprite::new(),
            move_animation: None,
            title_size: Vector2f::new(300.0, 300.0),
            buttons: ButtonsGroup::new()
        }
    }

    fn init_title_sprite(&mut self, title_texture: &RcTexture, win_size: (u32, u32)) {
        self.title_sprite.set_texture(title_texture, true);
        
        let texture_size = title_texture.size().as_other::<f32>();
        let title_size = self.title_size;
        self.title_sprite.set_scale((
            title_size.x / texture_size.x,
            title_size.y / texture_size.y
        ));
        self.title_sprite.set_position((
            (win_size.0 as f32 - title_size.x) / 2.0,
            (win_size.1 as f32 - title_size.y / 2.0)
        ));
    }

}

impl State<GameData> for MainMenuState {
    fn on_init(&mut self, state_data: &mut StateData<GameData>) {
        let title_texture = state_data.assets_manager.get_asset(AssetType::Texture, "title".to_string()).unwrap();
        let win_size = state_data.data.screen_size;
        self.init_title_sprite(title_texture, win_size);   

        let target_pos = Vector2f::new(
            (win_size.0 as f32 - self.title_size.x) / 2.0,
            (self.title_size.y) / 3.0
        );

        self.move_animation = Some(MoveToAnimation::new(self.title_sprite.position(), target_pos));
    
        
        let btn_start_texture = state_data.assets_manager.get_asset(AssetType::Texture, "start_btn".to_string()).unwrap();
        let btn_quit_texture = state_data.assets_manager.get_asset(AssetType::Texture, "quit_btn".to_string()).unwrap();
        let buttons_size = Vector2f::new(200.0, 100.0);

        let pos = (
            (win_size.0 as f32 - buttons_size.x) / 2.0,
            (win_size.1 as f32 - target_pos.y + self.title_size.y - buttons_size.y) / 2.0
        );

        let mut start_btn = Button::new(btn_start_texture, buttons_size);
        start_btn.set_position(pos);
        self.buttons.add_button("start", start_btn);

        let mut quit_btn = Button::new(btn_quit_texture, buttons_size);
        quit_btn.set_position((
            pos.0,
            pos.1 + buttons_size.y + 5.0
        ));
        self.buttons.add_button("quit", quit_btn);

    }

    

    fn on_event(&mut self, event: Event, state_data: &mut StateData<GameData>) -> Transition<GameData> {
        self.buttons.on_event(event);

        Transition::None
    }

    fn on_update(&mut self, state_data: &mut StateData<GameData>) -> Transition<GameData> {
        {
            let move_animation = self.move_animation.as_mut().unwrap();
            if !move_animation.has_finished() {
                let pos = move_animation.update(state_data.delta_time);
                self.title_sprite.set_position(pos);
            }
        }

        if self.buttons.get_button("start").is_clicked() {
            return Transition::Replace(Box::new(GameMenuState::new()));
        }
        if self.buttons.get_button("quit").is_clicked() {
            return Transition::Quit;
        }
        self.buttons.reset();
        Transition::None
    }

    fn on_render(&mut self, state_data: &mut StateData<GameData>, window: &mut dyn RenderTarget) -> bool {
        window.clear(state_data.data.clear_color);
        window.draw(&self.title_sprite);
        if self.move_animation.as_ref().unwrap().has_finished() {
            self.buttons.draw(window);
        }

        true
    }
}