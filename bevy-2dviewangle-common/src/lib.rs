use bevy::prelude::{Handle, Image, TextureAtlasLayout};

pub struct FieldInfo {
    pub actor: u64,
    pub action: u16,
    pub angle: String,
    pub image: Option<Handle<Image>>,
    pub atlas_layout: Option<Handle<TextureAtlasLayout>>,
}

pub trait ActorsTexturesLoader {
    fn get_all(&self)  -> Vec<FieldInfo>;
}