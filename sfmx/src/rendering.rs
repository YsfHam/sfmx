use crate::sfml_export::*;

pub fn get_quad(transform: &Transform, color: Color, tex_coords: FloatRect) -> [Vertex; 6] {
    let mut v = [Vertex::default(); 6];

    let bounds = transform.transform_rect(FloatRect::new(0.0, 0.0, 1.0, 1.0));

    v[0].position = Vector2f::new(bounds.left, bounds.top);
    v[0].color = color;
    v[0].tex_coords = Vector2f::new(tex_coords.left, tex_coords.top);

    v[1].position = Vector2f::new(bounds.left, bounds.top + bounds.height);
    v[1].color = color;
    v[1].tex_coords = Vector2f::new(tex_coords.left, tex_coords.top + tex_coords.height);

    v[2].position = Vector2f::new(bounds.left + bounds.width, bounds.top + bounds.height);
    v[2].color = color;
    v[2].tex_coords = Vector2f::new(tex_coords.left + tex_coords.width, tex_coords.top + tex_coords.height);

    v[3] = v[2];

    v[4].position = Vector2f::new(bounds.left + bounds.width, bounds.top);
    v[4].color = color;
    v[4].tex_coords = Vector2f::new(tex_coords.left + tex_coords.width, tex_coords.top);

    v[5] = v[0];

    v
}