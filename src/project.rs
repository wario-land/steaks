use uuid::Uuid;
use std::collections::HashMap;
use enum_primitive::FromPrimitive;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub stages: HashMap<StageId, Stage>,
    pub tilesets: HashMap<Uuid, Tileset>,
}

#[derive(Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Difficulty {
    Normal,
    Hard,
    SHard,
}

pub type StageId = (PassageKind, StageKind);

#[derive(Debug, Copy, Clone, Hash, Serialize, Deserialize, Eq, PartialEq)]
pub enum PassageKind {
    Entry    = 0x00,
    Emerald  = 0x01,
    Ruby     = 0x02,
    Topaz    = 0x03,
    Sapphire = 0x04,
    Golden   = 0x05,
}

#[derive(Debug, Copy, Clone, Hash, Serialize, Deserialize, Eq, PartialEq)]
pub enum StageKind {
    Stage1 = 0x00,
    Stage2 = 0x01,
    Stage3 = 0x02,
    Stage4 = 0x03,
    Boss   = 0x04,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stage {
    pub areas: HashMap<Uuid, Area>,
    pub warps: HashMap<Uuid, Warp>,
    pub time_limits: HashMap<Difficulty, usize>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tileset {
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Area {
    pub tileset: Uuid,
    pub entities: HashMap<Difficulty, Vec<Entity>>,
}

enum_from_primitive! {
    #[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
    pub enum WarpKind {
        Vortex   = 0x01,
        Screen   = 0x02,
        Pipe     = 0x03,
        Unknown1 = 0x04,
        Unknown2 = 0x05,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Warp {
    pub area: Uuid,
    pub kind: WarpKind,
    pub x1: u8,
    pub x2: u8,
    pub y1: u8,
    pub y2: u8,
    pub destination: Option<Uuid>,
    pub music: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub x: u8,
    pub y: u8,
    pub kind: u8,
}

