use itertools::Itertools;
use project::{Project, Stage, Tileset, Area, StageId, PassageKind, StageKind, Difficulty};
use std::collections::HashMap;
use std::io::{Result, Error, Cursor};
use uuid::Uuid;
use enum_primitive::FromPrimitive;
use rom_map::{Rom, RomStruct, TILESET_HEADERS, STAGE_HEADER_INDICES, STAGE_HEADERS, AREA_HEADER_POINTERS};
use byteorder::{LittleEndian, ReadBytesExt};
use gba_rs::compression::game_specific::wario_land_4::decompress_wl4_rle;
use nom::{le_u8, le_u32, IResult, ErrorKind};

pub fn rom_address(data: &[u8]) -> IResult<&[u8], u32> {
    let result = le_u32(data);
    if let IResult::Done(_, address) = result {
        if (address & 0xFF000000) != 0x08000000 {
            return IResult::Error(ErrorKind::Custom(0))
        }
    }
    result
}

pub fn import(path: &str) -> Result<Project> {
    let rom = Rom::new(path)?;

    let (tilesets, tilesets_uuid) = {
        let mut results_uuid = HashMap::new();
        let mut results = HashMap::new();
        for i in 0..92 {
            let tileset_header = {
                struct RomTilesetHeader {
                    pointer1: u32,
                    data1: u32,
                    pointer2: u32,
                    pointer3: u32,
                    data2: u32,
                    pointer4: u32,
                    pointer5: u32,
                    pointer6: u32,
                    pointer7: u32,
                }

                named!(rom_tileset_header<&[u8], RomTilesetHeader>,
                    do_parse!(
                        pointer1: rom_address >>
                        data1:    le_u32      >>
                        pointer2: rom_address >>
                        pointer3: rom_address >>
                        data2:    le_u32      >>
                        pointer4: rom_address >>
                        pointer5: rom_address >>
                        pointer6: rom_address >>
                        pointer7: rom_address >>
                        (RomTilesetHeader {
                            pointer1,
                            data1,
                            pointer2,
                            pointer3,
                            data2,
                            pointer4,
                            pointer5,
                            pointer6,
                            pointer7,
                        })
                    )
                );

                rom_tileset_header(rom.read_struct(&TILESET_HEADERS, i)?).unwrap().1
            };

            println!("{:?}", tileset_header.pointer1);

            let tileset = Tileset {

            };

            let uuid = Uuid::new_v4();
            results.insert(uuid, tileset);
            results_uuid.insert(i, uuid);
        }
        (results, results_uuid)
    };

    let stages = {
        let mut results = HashMap::new();

        let passage_variants = [PassageKind::Entry, PassageKind::Emerald, PassageKind::Ruby, PassageKind::Topaz, PassageKind::Sapphire, PassageKind::Golden];
        let stage_variants = [StageKind::Stage1, StageKind::Stage2, StageKind::Stage3, StageKind::Stage4, StageKind::Boss];
        let passage_iter = passage_variants.iter().cloned();
        let stage_iter = stage_variants.iter().cloned();

        for (passage_kind, stage_kind) in iproduct!(passage_iter, stage_iter) {
            let stage_opt = {
                let stage_header_indices_index = (passage_kind as usize) * 6 + (stage_kind as usize);
                let stage_header_index = rom.read_u32(STAGE_HEADER_INDICES.nth_address(stage_header_indices_index))? as usize;

                if stage_header_index == 0x32 {
                    None
                } else {
                    let stage_header = {
                        struct RomStageHeader {
                            area_pointer_index: u8,
                            area_count: u8,
                            time_hard: (u8, u8, u8),
                            time_normal: (u8, u8, u8),
                            time_shard: (u8, u8, u8),
                        }

                        named!(rom_stage_header<&[u8], RomStageHeader>,
                            do_parse!(
                                area_pointer_index: le_u8         >>
                                area_count:         le_u8         >>
                                                    tag!(&[0x0A]) >>
                                time_hard_1:        le_u8         >>
                                time_hard_2:        le_u8         >>
                                time_hard_3:        le_u8         >>
                                time_normal_1:      le_u8         >>
                                time_normal_2:      le_u8         >>
                                time_normal_3:      le_u8         >>
                                time_shard_1:       le_u8         >>
                                time_shard_2:       le_u8         >>
                                time_shard_3:       le_u8         >>
                                (RomStageHeader {
                                    area_pointer_index,
                                    area_count,
                                    time_hard: (time_hard_1, time_hard_2, time_hard_3),
                                    time_normal: (time_normal_1, time_normal_2, time_normal_3),
                                    time_shard: (time_shard_1, time_shard_2, time_shard_3),
                                })
                            )
                        );

                        rom_stage_header(rom.read_struct(&STAGE_HEADERS, stage_header_index)?).unwrap().1
                    };

                    let time_limits = {
                        let parse_time = |data: (u8, u8, u8)| {
                            (data.0 as usize) * 60 + (data.1 as usize) * 10 + (data.2 as usize)
                        };

                        let mut results = HashMap::new();
                        results.insert(Difficulty::Normal, parse_time(stage_header.time_normal));
                        results.insert(Difficulty::Hard, parse_time(stage_header.time_hard));
                        results.insert(Difficulty::SHard, parse_time(stage_header.time_shard));
                        results
                    };

                    let (areas, area_uuid) = {
                        let area_header_structs = RomStruct {
                            base_address: rom.read_u32(AREA_HEADER_POINTERS.nth_address(stage_header.area_pointer_index as usize))?,
                            struct_length: 44,
                        };

                        let mut results_uuid = HashMap::new();
                        let mut results = HashMap::new();

                        for i in 0..stage_header.area_count {
                            let area = Area {
                                tileset: Uuid::new_v4(), // TODO
                                entities: HashMap::new(), // TODO
                            };

                            let uuid = Uuid::new_v4();
                            results.insert(uuid, area);
                            results_uuid.insert(i, uuid);
                        }

                        (results, results_uuid)
                    };

                    let (warps, warps_uuid) = {
                        let mut results_uuid: HashMap<u32, Uuid> = HashMap::new();
                        let mut results = HashMap::new();
                        // TODO
                        (results, results_uuid)
                    };

                    Some(Stage {
                        areas,
                        warps,
                        time_limits,
                    })
                }
            };

            if let Some(stage) = stage_opt {
                results.insert((passage_kind, stage_kind), stage);
            }
        }

        results
    };

    Ok(Project {
        stages,
        tilesets,
    })
}
