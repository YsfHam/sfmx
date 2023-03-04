use crate::sfml_export::*;
use super::state_machine::{StateMachine, StateData, State};
use super::timer::Timer;

pub(crate) enum AppSignal {
    Quit,
    None
}

pub struct CursorSettings {
    pub cursor_type: CursorType,
    pub is_grabbed: bool,
    pub is_visible: bool
}

impl CursorSettings {
    pub fn default() -> Self {
        Self {
            cursor_type: CursorType::Arrow,
            is_grabbed: false,
            is_visible: true
        }
    }
}

pub struct AppData {
    pub win_size: (u32, u32),
    pub title: &'static str,
    pub win_style: Style,
    pub context_settings: ContextSettings,
    pub frame_rate: u32,
    pub enable_vsync: bool
}

impl Default for AppData {
    fn default() -> Self {

        let context_settings = ContextSettings::default();

        Self {
            win_size: (100, 100),
            title: "new Application",
            win_style: Style::DEFAULT,
            context_settings,
            frame_rate: 0,
            enable_vsync: true
        }
    }
}

pub struct AppBuilder<Data, S: State<Data>> {
    states_data: Option<StateData<Data>>,
    initial_state: Option<S>
}

impl<Data, S: State<Data> + 'static> AppBuilder<Data, S> {
    pub fn new() -> Self {
        Self {
            states_data: None,
            initial_state: None
        }
    }

    pub fn with_initial_state(mut self, initial_state: S) -> Self{
        self.initial_state = Some(initial_state);
        self
    }

    pub fn with_states_data(mut self, data: Data) -> Self {
        self.states_data = Some(StateData::new(data) );
        self
    }

    pub fn build(self, app_data: AppData) -> Application<Data> {
        let mut window = RenderWindow::new(
            app_data.win_size,
            app_data.title,
            app_data.win_style,
            &app_data.context_settings
        );
        window.set_framerate_limit(app_data.frame_rate);
        window.set_vertical_sync_enabled(app_data.enable_vsync);

        let mut states_data = self.states_data.expect("No data provided for the states");
        states_data.render_target_size = app_data.win_size;

        Application {
            window,
            cursor: None,
            state_machine: StateMachine::new(
                Box::new(self.initial_state.expect("Initial state is missing")),
                states_data
            )
        }
    }
}

pub struct Application<Data> {
    window: RenderWindow,
    cursor: Option<SfBox<Cursor>>,
    state_machine: StateMachine<Data>
}

impl<Data> Application<Data> {
    pub fn build<S: State<Data> + 'static>() -> AppBuilder<Data, S> {
        AppBuilder::new()
    }

    pub fn run(&mut self) {

        let timer = Timer::new();

        let mut current_time = timer.elapsed().as_secs_f32();
        while self.window.is_open() {

            let new_time = timer.elapsed().as_secs_f32();
            self.state_machine.states_data.delta_time = new_time - current_time;
            current_time = new_time;

            self.handle_events();
            self.update();

            self.render();

            if let Some(settings) = &self.state_machine.states_data.cursor {
                self.cursor = Cursor::from_system(settings.cursor_type);
                if let Some(cursor) = &self.cursor {
                    unsafe {
                        self.window.set_mouse_cursor(cursor);
                    }
                }
                self.window.set_mouse_cursor_grabbed(settings.is_grabbed);
                self.window.set_mouse_cursor_visible(settings.is_visible);
            }
            self.state_machine.states_data.cursor = None;

        }

        self.cleanup();
    }

    fn handle_events(&mut self) {
        while let Some(event) = self.window.poll_event() {
            if event == Event::Closed {
                self.window.close();
            }
            else {
                let app_signal = self.state_machine.on_event(event);
                self.handle_signal(app_signal);
            }
        }
    }

    fn update(&mut self) {
        let app_signal = self.state_machine.on_update();
        self.handle_signal(app_signal);
    }

    fn render(&mut self) {
       if self.state_machine.on_render(&mut self.window) {
            self.window.display();
       }
    }

    fn handle_signal(&mut self, signal: AppSignal) {
        match signal {
            AppSignal::Quit => self.window.close(),
            _ => {}
        }
    }

    fn cleanup(&mut self) {
        self.state_machine.terminate();
    }
}