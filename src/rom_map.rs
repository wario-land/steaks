#[derive(Debug)]
pub struct RomStruct {
    base_address: u32,
    struct_length: u32,
}

impl RomStruct {
    pub fn nth_address(&self, index: u32) -> u32 {
        self.base_address + self.struct_length * (index as u32)
    }
}

pub static STAGE_HEADER_INDICES: RomStruct = RomStruct {
    base_address: 0x086391C4,
    struct_length: 4,
};

pub static STAGE_HEADERS: RomStruct = RomStruct {
    base_address: 0x08639068,
    struct_length: 12,
};

pub static SFX_HEADERS: RomStruct = RomStruct {
    base_address: 0x08098028,
    struct_length: 8,
};

pub static TILESET_HEADERS: RomStruct = RomStruct {
    base_address: 0x083F2298,
    struct_length: 36,
};

#[macro_export]
macro_rules! rom {
    ($x:expr) => (
        {
            let addr: u32 = $x;
            if (addr & 0xFF000000) != 0x08000000 {
                Err(Error::new(ErrorKind::Other, "Not a ROM address"))
            } else {
                Ok((addr & 0x00FFFFFF) as usize)
            }
        }
    );
}
