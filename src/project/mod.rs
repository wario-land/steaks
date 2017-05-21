mod import;
mod export;
mod entity;
mod map;

pub use self::entity::Entity;
pub use self::map::Map;
pub use self::import::import;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub maps: Vec<Map>,
    // tilesets
}
