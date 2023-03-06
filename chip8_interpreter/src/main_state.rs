
use std::time::Duration;

use sfmx::prelude::*;
use crate::chip8::{Chip8, self};
use rfd::{FileDialog, MessageDialog};
use rodio::{source, Source, OutputStream};
use rodio;

pub struct MainState {
    chip8: Chip8,
    display_texture: SfBox<Texture>,
    texture_scale: f32,
    running_program: bool,
    ui_manager: UiManager<&'static str>,
    stream_handle: rodio::OutputStreamHandle,
    _stream: OutputStream,
    keyboard_mapping: [Key; 16],

    cycle_time: f32,
    dt: f32
}

impl MainState {
    pub fn new(texture_scale: f32) -> Self {
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

        let mut keyboard_mapping = [Key::Num0; 16];
       keyboard_mapping[chip8::Key::Key0 as usize] = Key::X;
       keyboard_mapping[chip8::Key::Key1 as usize] = Key::Num1;
       keyboard_mapping[chip8::Key::Key2 as usize] = Key::Num2;
       keyboard_mapping[chip8::Key::Key3 as usize] = Key::Num3;
       keyboard_mapping[chip8::Key::Key4 as usize] = Key::A;
       keyboard_mapping[chip8::Key::Key5 as usize] = Key::Z;
       keyboard_mapping[chip8::Key::Key6 as usize] = Key::E;
       keyboard_mapping[chip8::Key::Key7 as usize] = Key::Q;
       keyboard_mapping[chip8::Key::Key8 as usize] = Key::S;
       keyboard_mapping[chip8::Key::Key9 as usize] = Key::D;
       keyboard_mapping[chip8::Key::A as usize] =  Key::W;
       keyboard_mapping[chip8::Key::B as usize] =  Key::C;
       keyboard_mapping[chip8::Key::C as usize] =  Key::Num4;
       keyboard_mapping[chip8::Key::D as usize] =  Key::R;
       keyboard_mapping[chip8::Key::E as usize] =  Key::F;
       keyboard_mapping[chip8::Key::F as usize] =  Key::V;


        Self {
            chip8: Chip8::new(0x200),
            display_texture: Texture::new().unwrap(),
            texture_scale,
            running_program: false,
            ui_manager: UiManager::new(),
            stream_handle,
            _stream,
            keyboard_mapping,
            cycle_time: 1.0 / 60.0,
            dt: 0.0
        }
    }

    fn execute_next_instruction(&mut self) {
        if self.running_program {
            if let Err(_) = self.chip8.clock() {
                MessageDialog::new()
                .set_buttons(rfd::MessageButtons::Ok)
                .set_level(rfd::MessageLevel::Error)
                .set_title("Failed to run program")
                .set_description("Cannot continue to run the program.\nPlease make sure the program is for Chip-8")
                .show();

                self.chip8.reset();
            }
        }

        if self.chip8.play_sound() {
            let source = source::SineWave::new(440.0).amplify(0.2).take_duration(Duration::from_secs(1));
            self.stream_handle.play_raw(source).unwrap();
        }
    }

    fn update_screen(&mut self) {
        let display = self.chip8.get_display();
        if display.is_updated(){
            display.set_is_updated(false);
            let data = display.data().iter().map(|pixel| {
                if !self.running_program {
                    [rand::random(), rand::random(), rand::random(), 255u8]
                }
                else if *pixel == 0 {
                    [0u8, 0u8, 0u8, 255u8]
                }
                else {
                    [255u8, 255u8, 255u8, 255u8]
                }
            }).flatten().collect::<Vec<u8>>();
            unsafe {
                self.display_texture.update_from_pixels(&data, 
                    display.width() as u32, display.height() as u32, 0, 0);
            }
        }
    }

    fn draw_keyboard(&self, font: &SfBox<Font>, target: &mut dyn RenderTarget) {
        let mut text = DynamicText::new();
        text.set_font(&font);
        let cell_size = 50.0;
        for y in 0..4 {
            for x in 0..4 {
                let key = chip8::KEYBOARD_LAYOUT[y][x];
                text.set_position((x as f32 * cell_size, y as f32 * cell_size));
                text.move_((self.display_texture.size().x as f32 * self.texture_scale + 10.0, 10.0));
                text.set_string(key_to_string(key));
                if self.chip8.is_key_pressed(key) {
                    text.set_color(Color::GREEN);
                }
                else {
                    text.set_color(Color::WHITE);
                }
                target.draw(&text);
            }
        }
    }
}

impl State<()> for MainState {
    fn on_init(&mut self, state_data: &mut StateData<()>) {

        state_data.assets_manager.load_asset(AssetType::Font, "font".to_string(), "assets/slkscr.ttf");

        let font = state_data.assets_manager.get_asset(AssetType::Font, "font".to_string()).unwrap();

        if !self.display_texture.create(self.chip8.get_display().width() as u32,
             self.chip8.get_display().height() as u32) {
            println!("cannot create texture");
        }

        let mut load_btn = Button::new();
        load_btn.set_font(font);
        load_btn.set_size((100.0, 50.0).into());
        load_btn.set_color(Color::rgba(0, 0, 0, 0));
        load_btn.set_text("Load");
        load_btn.set_position((50.0, self.chip8.get_display().height() as f32 * self.texture_scale + 10.0).into());

        self.ui_manager.add_widget("load", load_btn);

        let mut load_btn = Button::new();
        load_btn.set_font(font);
        load_btn.set_size((100.0, 50.0).into());
        load_btn.set_color(Color::rgba(0, 0, 0, 0));
        load_btn.set_text("Help");
        load_btn.set_position((200.0, self.chip8.get_display().height() as f32 * self.texture_scale + 10.0).into());

        self.ui_manager.add_widget("help", load_btn);
    }

    fn on_event(&mut self, event: Event, _: &mut StateData<()>) -> Transition<()> {
        self.ui_manager.on_event(event);
        let mut key_state = None;
        if let Event::KeyPressed { code, ..} = event {
            key_state = Some((code, true));
        }
        else if let Event::KeyReleased { code, ..} = event {
            key_state = Some((code, false));
        }
        if let Some((code, b)) = key_state {
            for i in 0..self.keyboard_mapping.len() {
                if self.keyboard_mapping[i] == code {
                    println!("key {:?} pressed {}", code, b);
                    self.chip8.set_key_pressed((i as u8).into(), b);
                }
            }
        }

        Transition::None
    }

    fn on_update(&mut self, state_data: &mut StateData<()>) -> Transition<()> {

        //self.handle_input();
        if self.dt > self.cycle_time {
            self.execute_next_instruction();
            self.update_screen();
            self.dt = 0.0
        }
        else {
            self.dt += state_data.delta_time;
        }

        if self.ui_manager.get_widget(&"load").unwrap().is_clicked() {
            let file = FileDialog::new()
                .add_filter("chip8 exec", &["ch8"])
                .pick_file();
            if let Some(path) = file {
                self.running_program = true;
                self.chip8.reset();
                self.chip8.load_program(path.to_str().unwrap()).unwrap();
            }
        }

        if self.ui_manager.get_widget(&"help").unwrap().is_clicked() {
            MessageDialog::new()
            .set_description(
                "This is a Chip-8 emulator\nYou can choose a program to run with Load button\
                In the top right the Chip-8 keyboard layout, this layout is mapped to the following layout\
                \n1 2 3 4\nA Z E R\nQ S D F\nW X C V"
            )
            .set_buttons(rfd::MessageButtons::Ok)
            .set_title("Help")
            .set_level(rfd::MessageLevel::Info)
            .show();
        }

        self.ui_manager.reset();

        Transition::None
    }

    fn on_render(&mut self, state_data: &mut StateData<()>, target: &mut dyn RenderTarget) -> bool {
        target.clear(Color::BLUE);
        
        let mut sprite = Sprite::new();
        sprite.set_texture(&self.display_texture, true);
        sprite.set_scale((self.texture_scale, self.texture_scale));
        target.draw(&sprite);

        let font: &SfBox<Font> = state_data.assets_manager.get_asset(AssetType::Font, "font".to_string()).unwrap();
        self.draw_keyboard(font, target);
        
        self.ui_manager.draw(target);

        let mut text = DynamicText::new();
        text.set_font(font);
        text.set_char_size(25);
        text.set_size((200.0, 0.0));

        text.set_position((self.display_texture.size().x as f32 * self.texture_scale + 10.0, 280.0));
        text.set_string(&format!("Instruction\n\n{:#x}", self.chip8.get_last_instruction()));
        target.draw(&text);
        
        true
    }
    
}

fn key_to_string(key: chip8::Key) -> &'static str{
    match key {
        chip8::Key::Key0 => "0",
        chip8::Key::Key1 => "1",
        chip8::Key::Key2 => "2",
        chip8::Key::Key3 => "3",
        chip8::Key::Key4 => "4",
        chip8::Key::Key5 => "5",
        chip8::Key::Key6 => "6",
        chip8::Key::Key7 => "7",
        chip8::Key::Key8 => "8",
        chip8::Key::Key9 => "9",
        chip8::Key::A => "A",
        chip8::Key::B => "B",
        chip8::Key::C => "C",
        chip8::Key::D => "D",
        chip8::Key::E => "E",
        chip8::Key::F => "F",
    }
}

