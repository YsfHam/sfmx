
use sfmx::prelude::*;
use std::collections::HashMap;

pub struct Button {
    sprite: RcSprite,
    is_clicked: bool,
}

impl Button {
    pub fn new(texture: &RcTexture, size: impl Into<Vector2f>) -> Self {
        let mut sprite = RcSprite::with_texture(texture);

        let texture_size = texture.size().as_other::<f32>();
        let size = Vector2f::from(size.into());

        sprite.set_scale((
            size.x / texture_size.x,
            size.y / texture_size.y,
        ));

        Self {
            sprite,
            is_clicked: false
        }
    }

    pub fn set_rotation(&mut self, angle: f32) {
        self.sprite.set_rotation(angle);
    }

    pub fn on_event(&mut self, event: Event) -> bool{
        if let Event::MouseButtonReleased { button, x, y } = event {
            if button == mouse::Button::Left {
                self.is_clicked  = self.sprite.global_bounds().contains2(x as f32, y as f32);
                return self.is_clicked;
            }
        }
        false
    }

    pub fn is_clicked(&self) -> bool {
        self.is_clicked
    }

    fn reset(&mut self) {
        self.is_clicked = false;
    }

    pub fn draw(&self, target: &mut impl RenderTarget) {
        target.draw(&self.sprite);
    }

    pub fn set_position(&mut self, position: impl Into<Vector2f>) {
        self.sprite.set_position(position.into());
    }
}

pub struct ButtonsGroup {
    buttons: HashMap<String, Button>
}

impl ButtonsGroup {
    pub fn new() -> Self {
        Self {
            buttons: HashMap::new()
        }
    }

    pub fn add_button(&mut self, name: &str, button: Button) {
        self.buttons.insert(name.to_string(), button);
    }

    pub fn get_button(&self, name: &str) -> &Button {
        self.buttons.get(name).expect(&format!("Button {} not found", name))
    }


    pub fn reset(&mut self) {
        for (_, btn) in self.buttons.iter_mut() {
            btn.reset();
        }
    }

    pub fn on_event(&mut self, event: Event) -> bool {
        for (_, btn) in self.buttons.iter_mut() {
            if btn.on_event(event) {
                return true;
            }
        }
        false
    }

    pub fn draw(&self, target: &mut impl RenderTarget) {
        for (_, btn) in self.buttons.iter() {
            btn.draw(target);
        }
    }
}