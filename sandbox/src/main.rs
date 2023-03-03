#![allow(unused_variables)]
#![allow(dead_code)]

use sfmx::prelude::*;

struct TestState {
    dims: Vector2f,
    button: Button,

    typed_string: String,
}

impl TestState {
    fn new() -> Self {
        Self {
            dims: Vector2f::default(),
            button: Button::new(),
            typed_string: String::new()
        }
    }
}

type Data = ();

impl State<Data> for TestState {
    fn on_init(&mut self, state_data: &mut StateData<Data>) {
        state_data.assets_manager.load_asset(AssetType::Font, "font".to_string(), "/Library/Fonts/Arial.ttf");
        let font = state_data.assets_manager.get_asset(AssetType::Font, "font".to_string()).unwrap();
        self.button.set_text("Click me!");
        self.button.set_font(font);
        self.button.set_char_size(30);
        self.button.set_size((300.0, 50.0).into());
        self.button.set_position((100.0, 40.0).into());
        self.button.set_color(Color::RED); 

    }

    fn on_event(&mut self, event: Event, _: &mut StateData<Data>) -> Transition<Data> {
        self.button.on_event(event);

        if let Event::TextEntered { unicode } = event {
            self.typed_string.push(unicode);
        }

        Transition::None
    }

    fn on_update(&mut self, state_data: &mut StateData<Data>) -> Transition<Data> {

        if self.button.is_clicked() {
            println!("Button clicked !");
            let mut pos = self.button.position();
            pos.y = state_data.render_target_size.1 as f32 - 200.0;
            self.button.set_position(pos);
        }

        if self.button.is_hovered() {

            self.button.set_color(Color::GREEN);
        }
        else {
            self.button.set_color(Color::RED);
        }

        self.button.reset();

        Transition::None
    }

    fn on_render(&mut self, state_data: &mut StateData<Data>, window: &mut dyn RenderTarget) -> bool {

        window.clear(Color::BLACK);

        
        //self.button.draw(window);
        
        
        let font = state_data.assets_manager.get_asset(AssetType::Font, "font".to_string()).unwrap();
        let text = {
            let mut t = DynamicText::new();  
            t.set_string(&self.typed_string);
            t.set_font(&font);
            t.set_char_size(100);
            let size = Vector2f::new(state_data.render_target_size.0 as f32 / 2.0, state_data.render_target_size.1 as f32);
            t.set_size(size);
            t
        };
        window.draw(&text);


        true
    }
}
 
fn main() {

    let mut app_data = AppData::default();
    app_data.win_size = (500, 500);
    app_data.frame_rate = 60;

    let init_data = ();

    let init_state = TestState::new();

    Application::build()
        .with_initial_state(init_state)
        .with_states_data(init_data)
        .build(app_data)
        .run();
}
