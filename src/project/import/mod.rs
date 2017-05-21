//! Decompiles the given ROM into Steaks! project format

use std::io::{Cursor, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};
use project::{Project, Map, Entity};

pub fn import(rom_data: &[u8]) -> Result<Project, &'static str> {
    let mut project = Project {
        maps: Vec::new(),
    };

    for i in 0..242 {
        project.maps.push(import_map(rom_data, 0x3F4E88 + i * 44).unwrap());
    }

    Ok(project)
}

fn import_map(rom_data: &[u8], offset: usize) -> Result<Map, &'static str> {
    let tileset_index = rom_data[offset] as usize;

    let mut cursor = Cursor::new(rom_data);
    cursor.seek(SeekFrom::Start(offset as u64 + 0x1C)).unwrap();
    let offset_hard = cursor.read_u32::<LittleEndian>().unwrap() as usize & 0x00FFFFFF;
    let offset_normal = cursor.read_u32::<LittleEndian>().unwrap() as usize & 0x00FFFFFF;
    let offset_shard = cursor.read_u32::<LittleEndian>().unwrap() as usize & 0x00FFFFFF;

    let map = Map {
        tileset_index: tileset_index,
        entities_normal: import_entities(rom_data, offset_normal).unwrap(),
        entities_hard: import_entities(rom_data, offset_hard).unwrap(),
        entities_shard: import_entities(rom_data, offset_shard).unwrap(),
    };

    Ok(map)
}

fn import_entities(rom_data: &[u8], offset: usize) -> Result<Vec<Entity>, &'static str> {
    let mut result = Vec::new();

    let mut offset = offset;
    while let Some(entity) = import_entity(rom_data, offset) {
        result.push(entity);
        offset += 3;
    }

    Ok(result)
}

fn import_entity(rom_data: &[u8], offset: usize) -> Option<Entity> {
    let y = rom_data[offset];
    let x = rom_data[offset + 1];
    let kind = rom_data[offset + 2];

    if (x == 0xFF) && (y == 0xFF) && (kind == 0xFF) {
        None
    } else {
        Some(Entity{ x, y, kind })
    }
}
