use itertools::Itertools;
use project::{Project, Stage, Tileset, StageId, PassageKind, StageKind, Difficulty};
use std::collections::HashMap;
use std::io::{Result, Error, ErrorKind, Cursor};
use uuid::Uuid;
use enum_primitive::FromPrimitive;
use rom_map::{Rom, STAGE_HEADER_INDICES, STAGE_HEADERS};
use byteorder::{LittleEndian, ReadBytesExt};

pub fn import(rom: &Rom) -> Result<Project> {
    let project = Project {
        stages: import_stages(rom)?,
        tilesets: import_tilesets(rom)?,
    };
    Ok(project)
}

fn import_stages(rom: &Rom) -> Result<HashMap<StageId, Stage>> {
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
        let stage_opt = import_stage(rom, stage_id)?;
        if let Some(stage) = stage_opt {
            stages.insert(stage_id, stage);
        }
    }

    Ok(stages)
}

fn import_stage(rom: &Rom, stage_id: StageId) -> Result<Option<Stage>> {
    let (passage_kind, stage_kind) = stage_id;
    let stage_header_indices_index = (passage_kind as u32) * 6 + (stage_kind as u32);

    let address = STAGE_HEADER_INDICES.nth_address(stage_header_indices_index);
    let stage_headers_index = rom.read_u32(address)?;

    if stage_headers_index == 0x32 {
        Ok(None)
    } else {
        let address = STAGE_HEADERS.nth_address(stage_headers_index);
        let stage_header_data = rom.read_struct(&STAGE_HEADERS, stage_headers_index)?;
        println!("{:?}", stage_header_data);

        let areas = HashMap::new();
        let warps = HashMap::new();
        let time_limits = HashMap::new();

        Ok(Some(Stage {areas, warps, time_limits }))
    }
}

fn import_tilesets(rom: &Rom) -> Result<HashMap<Uuid, Tileset>> {
    Ok(HashMap::new())
}
