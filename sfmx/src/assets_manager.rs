use crate::sfml_export::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::collections::VecDeque;

pub type AssetID = String;

//////////////// Built-in assets type ////////////////////////
#[derive(Clone, Debug, Copy)]
pub enum AssetType {
    Texture = 0,
    Font,

    Count
}

impl Into<usize> for AssetType {
    fn into(self) -> usize {
        self as usize
    }
}
///////////////////////////////////////////////////

pub trait AssetsGroup {
    type AssetsId: Debug + Eq + Hash;
    fn has_next_load(&self) -> bool;
    fn get_next_load(&mut self) -> (Self::AssetsId, String);
    fn load(&mut self, id: Self::AssetsId, path: &str);
    fn load_buffered(&mut self, id: Self::AssetsId, path: &str);
    fn get_asset(&self, id: Self::AssetsId) -> Result<&dyn std::any::Any, String>;
}

pub struct DefaultAssetsGroup<ID, T, F: Fn(&'static str) -> T> {
    loader: F,
    storage: HashMap<ID, T>,
    buffered_loading: VecDeque<(ID, AssetID)>,
}

/////////////// Textures Assets ////////////////////

impl<ID, T, F: Fn(&str) -> T> DefaultAssetsGroup<ID, T, F> {
    fn new(loader: F) -> Self {
        Self {
            loader,
            storage: HashMap::new(),
            buffered_loading: VecDeque::new()
        }
    }
}

impl<ID: Debug + Eq + Hash + Clone, T: 'static, F: Fn(&str) -> T> AssetsGroup for DefaultAssetsGroup<ID, T, F> {
    type AssetsId = ID;    

    fn has_next_load(&self) -> bool {
        0 < self.buffered_loading.len()
    }

    fn get_next_load(&mut self) -> (ID, String) {
        let (name, path) = self.buffered_loading.pop_front().unwrap();

        (name, path)
    }

    fn get_asset(&self, id: Self::AssetsId) -> Result<&dyn std::any::Any, String> {

        match  self.storage.get(&id) {
            Some(asset) => Ok(asset),
            None => Err(format!("Asset with id {:?} is not found", id).into())
        }
    }

    fn load(&mut self, id: Self::AssetsId, path: &str) {
        let asset = (self.loader)(path);

        if self.storage.contains_key(&id) {
            panic!("Asset id {:?} already exists", id);
        }
        self.storage.insert(id, asset);
    }

    fn load_buffered(&mut self, id: Self::AssetsId, path: &str) {
        self.buffered_loading.push_back((id, path.to_string()))
    }


}

//////////////////////////////////////

pub struct AssetsManager<T> {
    //textures: TexturesAssets,
    assets_groups: Vec<Box<dyn AssetsGroup<AssetsId = T>>>,
    total_loadings: usize,
    remaining_loadings: usize
}

pub type DefaultAssetsManager = AssetsManager<AssetID>;

impl DefaultAssetsManager {
    pub fn default() -> Self {
        let mut res = AssetsManager::new();
        res.register_assets_group(Box::new(DefaultAssetsGroup::new(|p| RcTexture::from_file(p).unwrap())));
        res.register_assets_group(Box::new(DefaultAssetsGroup::new(|p| Font::from_file(p).unwrap())));
        res
    }
}

impl<T: Debug + Eq + Hash + Clone> AssetsManager<T> {

    pub fn new() -> Self {
        Self {
            assets_groups: Vec::new(),
            total_loadings: 0,
            remaining_loadings: 0
        }
    }

    pub fn register_assets_group(&mut self, assets_group: Box<dyn AssetsGroup<AssetsId = T>>) {
        self.assets_groups.push(
                assets_group
        );
    }

    pub fn load_asset<AT: Into<usize>>(&mut self, asset_type: AT, id: T, path: &str) {
        self.assets_groups[asset_type.into()].load(id, path);
    }

    pub fn load_asset_buffered<AT: Into<usize>>(&mut self, asset_type: AT, id: T, path: &str) {
        self.assets_groups[asset_type.into()].load_buffered(id, path);
        self.add_load_buffer();
    }

    pub fn get_asset<A: 'static, AT: Into<usize>>(&self, asset_type: AT, id: T) -> Result<&A, String> {
        let any_value = self.assets_groups[asset_type.into()].get_asset(id)?;

        match any_value.downcast_ref::<A>() {
            Some(a) => Ok(a),
            None => Err(format!("Asset have incompatible type"))
        }
    }

    fn add_load_buffer(&mut self) {
        if self.remaining_loadings == 0 {
            self.total_loadings = 0;   
        }
        self.total_loadings += 1;
        self.remaining_loadings += 1;
    }

    pub fn launch_loadings(&mut self) {

        if self.remaining_loadings == 0 {
            return;
        }
        let mut i = 0;

        while i < self.assets_groups.len() && !self.assets_groups[i].has_next_load() {
            i += 1;
        }
        if i >= self.assets_groups.len() {
            return;
        }

        let (name, path) = self.assets_groups[i].get_next_load();
        self.load_asset(i, name, &path);

        self.remaining_loadings -= 1;
    }

    pub fn loading_percentage(&self) -> f32 {
        let done = self.total_loadings - self.remaining_loadings;
        done as f32 / self.total_loadings as f32 * 100.0
    }
}