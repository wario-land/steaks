use project::Entity;

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    pub tileset: usize,
    pub entities_normal: Vec<Entity>,
    pub entities_hard: Vec<Entity>,
    pub entities_shard: Vec<Entity>,
}
