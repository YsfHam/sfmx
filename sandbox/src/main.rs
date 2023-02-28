use sfmx::prelude::*;

#[derive(Default)]
struct Player {
    velocity: Vector2f,
    sprite: RcSprite
}

impl Player {
    fn new() -> Self {

        Self {
            velocity: Vector2f::new(100.0, 100.0),
            sprite: RcSprite::new(),
            ..Default::default()
        }
    }

    fn set_texture(&mut self, texture: &RcTexture) {
        self.sprite.set_texture(texture, true);
        let texture_size = texture.size().as_other::<f32>();
        let sprite_size = Vector2f::new(50.0, 50.0);
        self.sprite.scale((
            sprite_size.x / texture_size.x,
            sprite_size.y / texture_size.y
        ));

    }

    fn update_position(&mut self, dt: f32, dir: Vector2f) {
        let vel = Vector2f::new(self.velocity.x * dir.x, self.velocity.y * dir.y);
        self.sprite.move_(vel * dt);
    }

    fn get_sprite(&self) -> &RcSprite {
        &self.sprite
    }
}

struct ProgressBar {
    bar_sprite: RcSprite,
    bar_size: Vector2f
}

impl ProgressBar {
    fn new(texture: &RcTexture, bar_size: impl Into<Vector2f>, color: Color) -> Self {
        let mut sprite = RcSprite::new();
        sprite.set_texture(texture, true);
        sprite.set_color(color);

        let texture_size = texture.size().as_other::<f32>();
        let bar_size = bar_size.into();
        sprite.scale((
            0.0,
            bar_size.y / texture_size.y
        ));

        Self {
            bar_sprite: sprite,
            bar_size
        }
    }

    fn set_position(&mut self, pos: impl Into<Vector2f>) {
        self.bar_sprite.set_position(pos);
    }

    fn set_progress(&mut self, progress: f32) {
        let progress = progress.clamp(0.0, 100.0);
        let new_width = self.bar_size.x * progress / 100.0;
        let current_scale = self.bar_sprite.get_scale();
        let texture_width = self.bar_sprite.texture().unwrap().size().x as f32;
        self.bar_sprite.set_scale((new_width / texture_width, current_scale.y));
    }
}

struct TestState {
    player: Option<Player>,
    camera: SfBox<View>,
    progress_bar: Option<ProgressBar>,
}

impl TestState {
    fn new(window_size: Vector2f) -> Self {
        Self {
            player: None,
            camera: View::new(window_size / 2.0, window_size),
            progress_bar: None,
        }
    }
}

type Data = ();

impl State<Data> for TestState {
    fn on_init(&mut self, state_data: &mut StateData<Data>) {

        state_data.assets_manager.
        load_asset(AssetType::Texture, "awesome_face".to_string(), "assets/awesomeface.png");
        let mut player = Player::new();
        player.set_texture(state_data.assets_manager.get_asset(AssetType::Texture, "awesome_face".to_string()).unwrap());
        self.player = Some(player);

        state_data.assets_manager.
        load_asset(AssetType::Texture, "white_texture".to_string(), "assets/white_texture.png");

        let white_texture = state_data.assets_manager.get_asset(AssetType::Texture, "white_texture".to_string()).unwrap();
        let x = state_data.assets_manager.get_asset::<Font, _>(AssetType::Font, "font".to_string()).unwrap();

        let mut progress_bar = ProgressBar::new(&white_texture, (400.0, 10.0), Color::RED);
        progress_bar.set_position((50.0, 300.0));
        self.progress_bar = Some(progress_bar);


        for i in 0..1000 {
            state_data.assets_manager.
            load_asset_buffered(AssetType::Texture, format!("test_loading{}", i), "assets/awesomeface.png")
        }

    }

    fn on_event(&mut self, event: Event, _: &mut StateData<Data>) -> Transition<Data> {

        if let Event::Resized { width, height } = event {
            let new_size = Vector2f::new(width as f32, height as f32);
            self.camera.set_size(new_size);
            self.camera.set_center(new_size / 2.0);
        }

        Transition::None
    }

    fn on_update(&mut self, state_data: &mut StateData<Data>) -> Transition<Data> {

        state_data.assets_manager.launch_loadings();

        let mut dir = Vector2f::default();
        if Key::Z.is_pressed() {
            dir.y = -1.0;
        }
        if Key::S.is_pressed() {
            dir.y = 1.0;
        }

        if Key::Q.is_pressed() {
            dir.x = -1.0;
        }
        if Key::D.is_pressed() {
            dir.x = 1.0;
        }

        let length_sq = dir.length_sq();
        if length_sq > 1.0 {
            dir = dir / length_sq.sqrt();
        }
        self.player.as_mut().unwrap().update_position(state_data.delta_time, dir);

        // if Key::P.is_pressed() {
        //     self.progress += 10.0 * state_data.delta_time;
        //     
        // }

        self.progress_bar.as_mut().unwrap().set_progress(state_data.assets_manager.loading_percentage());

        Transition::None
    }

    fn on_render(&mut self, _state_data: &mut StateData<Data>, window: &mut dyn RenderTarget) -> bool {

        window.clear(Color::BLACK);

        window.set_view(&self.camera);

        //draw_map(MAP, window);
        
        window.draw(self.player.as_mut().unwrap().get_sprite());
        window.draw(&self.progress_bar.as_ref().unwrap().bar_sprite);
        true
    }


}
 
fn main() {

    let mut app_data = AppData::default();
    app_data.win_size = (500, 500);
    app_data.frame_rate = 60;

    let init_data = ();

    let init_state = TestState::new(Vector2f::new(
        app_data.win_size.0 as f32,
        app_data.win_size.1 as f32
    ));

    Application::build()
        .with_initial_state(init_state)
        .with_states_data(init_data)
        .build(app_data)
        .run();
}
