

use crate::sfml_export::*;
use crate::rendering::get_quad;

#[derive(Default)]
struct TextTransform {
    position: Vector2f,
    rotation: f32,
    scale: Vector2f,
    origin: Vector2f,
    inverse_transform: Transform,
    transform: Transform
}

impl TextTransform {
    fn default() -> Self {
        Self {
            scale: Vector2f::new(1.0, 1.0),
            ..Default::default()
        }
    }
}


pub struct DynamicText {
    text: String,
    font: Option<SfBox<Font>>,
    char_size: u32,
    transform: TextTransform,
    color: Color,
    size: Vector2f
}

impl DynamicText {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            font: None,
            char_size: 30,
            transform: TextTransform::default(),
            color: Color::WHITE,
            size: Vector2f::default()
        }
    }

    pub fn set_size(&mut self, size: impl Into<Vector2f>) {
        self.size = size.into();
    }

    pub fn size(&self) -> Vector2f {
        self.size
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn set_font(&mut self, font: &SfBox<Font>) {
        self.font = Some(font.clone());
    }

    pub fn set_string(&mut self, string: &str) {
        self.text = string.to_string();
    }

    pub fn set_char_size(&mut self, char_size: u32) {
        self.char_size = char_size;
    }

    pub fn global_bounds(&self) -> FloatRect {
        let mut nb_lines = 0.0;
        let mut advance = 0.0;
        let text_zone_size = self.size;

        let mut occupied_size = Vector2f::default();

        let font = self.font.as_ref().unwrap();

        for c in self.text.chars() {

            if c == '\n' {
                nb_lines += 1.0;
                advance = 0.0;
                continue;
            }
    
            if c == '\r' {
                continue;
            }
    
            let glyph = font.glyph(c as u32, self.char_size, false, 0.0);
            let bounds = glyph.bounds();
            let line_spacing = font.line_spacing(self.char_size);
    
            let mut pos = Vector2f::new(advance, bounds.height + bounds.top + nb_lines * line_spacing);
            let size = Vector2f::new(bounds.width, bounds.height);

            if pos.x + size.x > text_zone_size.x  && text_zone_size.x > 0.0{
                nb_lines += 1.0;
                pos.y += line_spacing;
                pos.x = 0.0;
                advance = 0.0;
            }
            else {
                occupied_size.x = pos.x + size.x * self.transform.scale.x;
            }
    
            if pos.y + size.y > text_zone_size.y && text_zone_size.y > 0.0{
                break;
            }
            else {
                occupied_size.y = pos.y + size.y * self.transform.scale.y;
            }
            advance += glyph.advance();
        }
        FloatRect::from_vecs(self.position(), occupied_size)
    }

    fn update_transform(&mut self) {
        let mut transform = Transform::default();
        transform.translate(self.transform.position.x, self.transform.position.y);
        transform.rotate_with_center(self.transform.rotation, self.transform.origin.x, self.transform.origin.y);
        transform.scale_with_center(self.transform.scale.x, self.transform.scale.y, self.transform.origin.x, self.transform.origin.y);
        self.transform.transform = transform;
        self.transform.inverse_transform = transform.inverse();
    }
}

impl Transformable for DynamicText {
    fn get_scale(&self) -> Vector2f {
        self.transform.scale
    }

    fn move_<O: Into<Vector2f>>(&mut self, offset: O) {
        self.transform.position += offset.into();  
        self.update_transform(); 
    }

    fn origin(&self) -> Vector2f {
        self.transform.origin
    }

    fn inverse_transform(&self) -> &Transform {
        &self.transform.inverse_transform
    }

    fn position(&self) -> Vector2f {
        self.transform.position
    }

    fn rotate(&mut self, angle: f32) {
        self.transform.rotation += angle;
        self.update_transform();
    }

    fn rotation(&self) -> f32 {
        self.transform.rotation
    }

    fn scale<F: Into<Vector2f>>(&mut self, factors: F) {
        let factors = factors.into();
        self.transform.scale.x *= factors.x;
        self.transform.scale.y *= factors.y;

        self.update_transform();
    }

    fn set_origin<O: Into<Vector2f>>(&mut self, origin: O) {
        self.transform.origin = origin.into()
    }

    fn set_position<P: Into<Vector2f>>(&mut self, position: P) {
        self.transform.position = position.into();

        self.update_transform();
    }

    fn set_rotation(&mut self, angle: f32) {
        self.transform.rotation = angle;

        self.update_transform();
    }

    fn set_scale<S: Into<Vector2f>>(&mut self, scale: S) {
        self.transform.scale = scale.into();

        self.update_transform();
    }

    fn transform(&self) -> &Transform {
        &self.transform.transform
    }

}

impl Drawable for DynamicText {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
            &'a self,
            target: &mut dyn RenderTarget,
            states: &RenderStates<'texture, 'shader, 'shader_texture>,
        ) {
        let mut states = states.clone();
        states.set_texture(Some(self.font.as_ref().unwrap().texture(self.char_size)));
        let vertices = 
            gen_glyphs_vertices(self);
        target.draw_primitives(&vertices, PrimitiveType::TRIANGLES, &states);
    }
}



fn gen_glyphs_vertices(dynamic_text: &DynamicText) -> Vec<Vertex>{
    let mut vertices = Vec::new();
    let mut advance = 0.0;
    let mut nb_lines = 0.0;

    let font = dynamic_text.font.as_ref().unwrap();
    let text = &dynamic_text.text;
    let char_size= dynamic_text.char_size;
    let color= dynamic_text.color;
    let text_zone_size = dynamic_text.size;
    let transform = &dynamic_text.transform;

    let h_bounds = font.glyph('h' as u32, char_size, false, 0.0).bounds();

    for c in text.chars() {

        if c == '\n' {
            nb_lines += 1.0;
            advance = 0.0;
            continue;
        }

        if c == '\r' {
            continue;
        }

        let glyph = font.glyph(c as u32, char_size, false, 0.0);
        let bounds = glyph.bounds();
        let line_spacing = font.line_spacing(char_size);
        
        let mut height = bounds.height + bounds.top;
        if height < h_bounds.height - bounds.height {
            height = h_bounds.height - bounds.height;
        }
        let mut pos = Vector2f::new(advance, height + nb_lines * line_spacing);
        let size = Vector2f::new(bounds.width * transform.scale.x, bounds.height * transform.scale.y);
        let texture_rect = glyph.texture_rect().as_other::<f32>();

        if pos.x + size.x > text_zone_size.x  && text_zone_size.x > 0.0{
            nb_lines += 1.0;
            pos.y += line_spacing;
            pos.x = 0.0;
            advance = 0.0;
        }

        if pos.y + size.y > text_zone_size.y && text_zone_size.y > 0.0{
            break;
        }


        pos.x += transform.position.x - transform.origin.x;
        //pos.y += transform.position.y - transform.origin.y;
        
        let mut glyph_trans = Transform::default();
        glyph_trans.translate(pos.x, pos.y);
        glyph_trans.scale(size.x, size.y);
        glyph_trans.rotate_with_center(dynamic_text.rotation(), transform.origin.x, transform.origin.y);


        let quad = get_quad(&glyph_trans, color, texture_rect);
        advance += glyph.advance();

        vertices.extend_from_slice(&quad);
    }
    vertices
}