use itertools::Itertools;
use project::{Project, Stage, Tileset, StageId, PassageKind, StageKind};
use std::collections::HashMap;
use std::io::{Result, Error, ErrorKind, Cursor};
use uuid::Uuid;
use enum_primitive::FromPrimitive;
use rom_map::{STAGE_HEADER_INDICES, STAGE_HEADERS};

pub fn import(rom_data: &[u8]) -> Result<Project> {
    let project = Project {
        stages: import_stages(rom_data)?,
        tilesets: import_tilesets(rom_data)?,
    };
    Ok(project)
}

fn import_stages(rom_data: &[u8]) -> Result<HashMap<StageId, Stage>> {
    // TODO: Find a way to get these enum variants more sophistically
    // The current implementation is prone to errors.
    use self::PassageKind::*;
    use self::StageKind::*;
    let passage_variants = [Entry, Emerald, Ruby, Topaz, Sapphire, Golden];
    let stage_variants = [Stage1, Stage2, Stage3, Stage4, Boss];

    let mut stages = HashMap::new();

    let passage_iter = passage_variants.iter().cloned();
    let stage_iter = stage_variants.iter().cloned();
    for stage_id in iproduct!(passage_iter, stage_iter) {
        let stage_opt = import_stage(rom_data, stage_id)?;
        if let Some(stage) = stage_opt {
            stages.insert(stage_id, stage);
        }
    }

    Ok(stages)
}

fn import_stage(rom_data: &[u8], stage_id: StageId) -> Result<Option<Stage>> {
    let (passage_kind, stage_kind) = stage_id;
    let stage_index = (passage_kind as u32) * 6 + (stage_kind as u32);
    let address = STAGE_HEADER_INDICES.nth_address(stage_index);

    println!("0x{:08X}", address);
    println!("{:?}", rom_data[rom!(address)?]);

    Ok(Some(Stage {
        areas: HashMap::new(),
        warps: HashMap::new(),
        time_limits: HashMap::new(),
    }))

}

fn import_tilesets(rom_data: &[u8]) -> Result<HashMap<Uuid, Tileset>> {
    Ok(HashMap::new())
}


