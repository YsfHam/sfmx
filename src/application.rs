use crate::sfml_export::*;
use crate::state_machine::{StateMachine, StateData, State};
use crate::timer::Timer;

pub(crate) enum AppSignal {
    Quit,
    None
}

pub struct AppData {
    pub win_size: (u32, u32),
    pub title: &'static str,
    pub win_style: Style,
    pub context_settings: ContextSettings,
    pub frame_rate: u32
}

impl Default for AppData {
    fn default() -> Self {

        let context_settings = ContextSettings::default();

        Self {
            win_size: (100, 100),
            title: "new Application",
            win_style: Style::DEFAULT,
            context_settings,
            frame_rate: 0
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
        let states_data = self.states_data.expect("No data provided for the states");

        Application {
            window,
            state_machine: StateMachine::new(
                Box::new(self.initial_state.expect("Initial state is missing")),
                states_data
            )
        }
    }
}

pub struct Application<Data> {
    window: RenderWindow,
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
        self.state_machine.on_render(&mut self.window);
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