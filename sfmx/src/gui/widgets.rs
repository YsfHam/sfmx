use crate::{sfml_export::*, prelude::DynamicText};

pub trait Widget {
    fn position(&self) -> Vector2f;
    fn set_position(&mut self, position: Vector2f);
    fn size(&self) -> Vector2f;
    fn set_size(&mut self, size: Vector2f);

    fn is_clicked(&self) -> bool;
    fn is_hovered(&self) -> bool;

    fn on_event(&mut self, event: Event);

    fn reset(&mut self);

    fn draw(&self, target: &mut dyn RenderTarget);
}

pub struct Button {
    clicked: bool,
    hovered: bool,
    last_mouse_pos: Vector2f,
    
    position: Vector2f,
    size: Vector2f,
    color: Color,
    
    text: DynamicText,

    offset_from_text: Vector2f
}

impl Button {
    pub fn new() -> Box<Self> {
        Box::new(
            Self {
                clicked: false,
                hovered: false,
                last_mouse_pos: Vector2f::default(),

                position: Vector2f::default(),
                size: Vector2f::default(),
                color: Color::WHITE,

                text: DynamicText::new(),

                offset_from_text: Vector2f::default(),
            }
        )
    }

    pub fn with_text(text: &str) -> Box<Self> {
        let mut btn = Self::new();
        btn.text.set_string(text);
        btn
    }

    pub fn set_text(&mut self, text: &str) {
        self.text.set_string(text);
    }

    pub fn set_char_size(&mut self, char_size: u32) {
        self.text.set_char_size(char_size);
    }

    pub fn set_font(&mut self, font: &SfBox<Font>) {
        self.text.set_font(font);
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn set_offset_from_text(&mut self, offset_from_text: Vector2f) {
        self.offset_from_text = offset_from_text;
    }

    fn update_text(&mut self) {
        let text = &mut self.text;
        let text_bounds = text.global_bounds();
        text.set_size(self.size - self.offset_from_text);
        text.set_position((
            (self.size.x / 2.0 - text_bounds.width / 2.0),
            self.size.y / 2.0 - text_bounds.height / 2.0
        ));
        text.move_(self.position);
    }

}

impl Widget for Button {
    fn draw(&self, target: &mut dyn RenderTarget) {

        let btn_rect = {
            let mut rect = RectangleShape::with_size(self.size);
            rect.set_position(self.position);
            rect.set_fill_color(self.color);
            rect
        };
        target.draw(&btn_rect);
        target.draw(&self.text);
    }

    fn is_clicked(&self) -> bool {
        self.clicked
    }

    fn is_hovered(&self) -> bool {
        self.hovered
    }

    fn position(&self) -> Vector2f {
        self.position
    }

    fn set_position(&mut self, position: Vector2f) {
        self.position = position;

        self.update_text();
    }

    fn set_size(&mut self, size: Vector2f) {
        self.size = size;
        self.update_text();
    }

    fn size(&self) -> Vector2f {
        self.size
    }

    fn reset(&mut self) {
        let bounds = FloatRect::from_vecs(self.position, self.size);
        self.clicked = false;
        self.hovered = bounds.contains(self.last_mouse_pos);
    }

    fn on_event(&mut self, event: Event) {
        let bounds = FloatRect::from_vecs(self.position, self.size);
        match event {
            Event::MouseButtonPressed { button, x, y } => {
                if button == mouse::Button::Left &&  bounds.contains2(x as f32, y as f32) {
                    self.clicked = true;
                }
            },
            Event::MouseMoved { x, y } => {
                let x = x as f32;
                let y = y as f32;
                self.last_mouse_pos = Vector2f::new(x, y);
                self.hovered = bounds.contains2(x, y);
            }
            _ => {}
        }
    }
}