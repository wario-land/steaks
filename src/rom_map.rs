use std::io::{Read, Result, Cursor, Error, ErrorKind};
use std::fs::File;
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};

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

pub struct Rom {
    pub data: Vec<u8>,
}

impl Rom {
    pub fn new(path: &str) -> Result<Self> {
        let mut data: Vec<u8> = Vec::new();
        let mut file = File::open(path)?;
        file.read_to_end(&mut data)?;
        Ok(Self { data })
    }

    pub fn read_u16(&self, address: u32) -> Result<u16> {
        Ok(LittleEndian::read_u16(&self.data[rom!(address)?..]))
    }

    pub fn read_u32(&self, address: u32) -> Result<u32> {
        Ok(LittleEndian::read_u32(&self.data[rom!(address)?..]))
    }

    pub fn read_struct(&self, rom_struct: &RomStruct, index: u32) -> Result<&[u8]> {
        let address = rom!(rom_struct.nth_address(index))?;
        let length = rom_struct.struct_length as usize;
        Ok(&self.data[address..address + length])
    }

    pub fn slice_from(&self, address: u32) -> Result<&[u8]> {
        Ok(&self.data[rom!(address)?..])
    }
}

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
