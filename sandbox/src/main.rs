use sfmx::prelude::*;

const MAP:&[&'static str] = &[
    "##################",
    "#    -      -    #",
    "#    -      -    #",
    "#    -      -    #",
    "#    -      -    #",
    "##################",
];

fn draw_map(map: &[&'static str], target: &mut impl RenderTarget) {

    let map_size = Vector2f::new(map[0].len() as f32, map.len() as f32);
    let target_size = target.size().as_other::<f32>();
    let quad_size = Vector2f::new(target_size.x / map_size.x, target_size.y / map_size.y);

    
    if Key::K.is_pressed() {
        println!("Window size {:?}", target_size);
        println!("Quad size {:?}", quad_size);
        println!("map_size {:?}", map_size);
    }

    let mut bloc = RectangleShape::new();
    bloc.set_size(quad_size);

    for y in 0..map_size.y as usize {
        for x in 0..map_size.x as usize {
            match map[y].as_bytes()[x] as char {
                '#' => {
                    bloc.set_fill_color(Color::RED);
                }
                '-' => {
                    bloc.set_fill_color(Color::BLUE);
                }
                _ => continue
            }
            bloc.set_position((x as f32 * quad_size.x, y as f32 * quad_size.y));
            target.draw(&bloc);
        }
    }
}

#[derive(Default)]
struct Player {
    position: Vector2f,
    velocity: Vector2f,
    size: Vector2f,
    sprite: RcSprite
}

impl Player {
    fn new() -> Self {
        Self {
            size: Vector2f::new(50.0, 50.0),
            velocity: Vector2f::new(100.0, 100.0),
            sprite: RcSprite::new(),
            ..Default::default()
        }
    }

    fn update_position(&mut self, dt: f32, dir: Vector2f) {
        let vel = Vector2f::new(self.velocity.x * dir.x, self.velocity.y * dir.y);
        self.position += vel * dt;
    }

    fn set_texture(&mut self, texture: &RcTexture) {
        self.sprite.set_texture(texture, true);
        self.sprite.set_position(self.position);
        self.sprite.set_scale((
            self.size.x / texture.size().x as f32,
            self.size.y / texture.size().y as f32
        ));
    }

    fn get_sprite(&mut self, texture: &RcTexture) -> &RcSprite {
        self.set_texture(texture);
        &self.sprite
    }
}

struct TestState {
    player: Option<Player>,
    camera: SfBox<View>
}

impl TestState {
    fn new(window_size: Vector2f) -> Self {
        Self {
            player: None,
            camera: View::new(window_size / 2.0, window_size)
        }
    }
}

type Data = ();

impl State<Data> for TestState {


    fn on_init(&mut self, state_data: &mut StateData<Data>) {

        state_data.assets_manager
            .load_textures("awesome_face", "assets/awesomeface.png")
            .unwrap();

        self.player = Some(Player::new());        
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

        

        Transition::None
    }

    fn on_render(&mut self, _state_data: &mut StateData<Data>, window: &mut RenderWindow) {

        window.clear(Color::BLACK);

        window.set_view(&self.camera);

        draw_map(MAP, window);

        let texture = _state_data.assets_manager
            .get_texture("awesome_face")
            .unwrap();

        window.draw(self.player.as_mut().unwrap().get_sprite(&texture));

        window.display();
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
