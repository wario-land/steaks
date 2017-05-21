use project::Entity;

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    pub tileset: usize,
    pub normal_entities: Vec<Entity>,
    pub hard_entities: Vec<Entity>,
    pub shard_entities: Vec<Entity>,
}
