//! Decompiles the given ROM into Steaks! project format

use std::io::{Read, Cursor, Seek, SeekFrom};
use byteorder::{LittleEndian, ReadBytesExt};
use project::{Project, Map, Entity};

pub fn decompile(rom_data: &[u8]) -> Result<Project, &'static str> {
    let mut project = Project {
        maps: Vec::new(),
    };

    for i in 0..242 {
        project.maps.push(decompile_map(rom_data, 0x3F4E88 + i * 44).unwrap());
    }

    Ok(project)
}

fn decompile_map(rom_data: &[u8], offset: usize) -> Result<Map, &'static str> {
    let tileset = rom_data[offset] as usize;

    let mut cursor = Cursor::new(rom_data);
    cursor.seek(SeekFrom::Start(offset as u64 + 0x1C));
    let offset_hard = cursor.read_u32::<LittleEndian>().unwrap() as usize & 0x00FFFFFF;
    let offset_normal = cursor.read_u32::<LittleEndian>().unwrap() as usize & 0x00FFFFFF;
    let offset_shard = cursor.read_u32::<LittleEndian>().unwrap() as usize & 0x00FFFFFF;

    let map = Map {
        tileset: tileset,
        entities_normal: decompile_entities(rom_data, offset_normal).unwrap(),
        entities_hard: decompile_entities(rom_data, offset_hard).unwrap(),
        entities_shard: decompile_entities(rom_data, offset_shard).unwrap(),
    };

    Ok(map)
}

fn decompile_entities(rom_data: &[u8], offset: usize) -> Result<Vec<Entity>, &'static str> {
    let mut result = Vec::new();

    let mut offset = offset;
    while let Some(entity) = decompile_entity(rom_data, offset) {
        result.push(entity);
        offset += 3;
    }

    Ok(result)
}

fn decompile_entity(rom_data: &[u8], offset: usize) -> Option<Entity> {
    let y = rom_data[offset];
    let x = rom_data[offset + 1];
    let kind = rom_data[offset + 2];

    if (x == 0xFF) && (y == 0xFF) && (kind == 0xFF) {
        None
    } else {
        Some(Entity{ x, y, kind })
    }
}
