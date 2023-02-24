use crate::sfml_export::*;
use std::collections::HashMap;

pub struct AssetsManager {
    textures: HashMap<String, RcTexture>
}

impl AssetsManager {
    pub fn new() -> AssetsManager {
        Self {
            textures: HashMap::new()
        }
    }

    pub fn load_textures(&mut self, name: &str, path: &str) -> Result<&RcTexture, ResourceLoadError> {
        let texture = RcTexture::from_file(path)?;
    
        self.textures.insert(name.to_string(), texture.to_owned());

        Ok(self.get_texture(name).unwrap())
    }

    pub fn get_texture(&mut self, name: &str) -> Result<&RcTexture, String> {
        match  self.textures.get(name) {
            Some(texture) => Ok(&texture),
            None => Err(format!("Texture {} not found", name).into())
        }
    }
}