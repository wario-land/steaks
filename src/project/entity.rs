#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub x: u8,
    pub y: u8,
    pub kind: u8, // TODO: Enum
}
