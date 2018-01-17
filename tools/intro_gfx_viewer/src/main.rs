extern crate minifb;
extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate gba_compression;
extern crate byteorder;

use minifb::{Key, WindowOptions, Window, Scale};
use structopt::StructOpt;
use std::num::ParseIntError;
use std::str::FromStr;
use std::io::{Read, Cursor};
use std::fs::File;
use gba_compression::bios::decompress_lz77;
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};

struct HexOffset(u32);

impl FromStr for HexOffset {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = u32::from_str_radix(s, 16)? & 0x00FFFFFF;
        Ok(HexOffset(value))
    }
}

#[derive(Copy, Clone, Debug)]
struct Rgb555(u16);

impl Rgb555 {
    fn argb8888(&self) -> u32 {
        ((((self.0 as u32 >>  0) & 0x1F) << 3) << 16) | // Red
        ((((self.0 as u32 >>  5) & 0x1F) << 3) <<  8) | // Green
        ((((self.0 as u32 >> 10) & 0x1F) << 3) <<  0) | // Blue
        0xFF000000 // Alpha
    }
}

#[derive(Copy, Clone, Debug)]
struct Tile(u16);

impl Tile {
    fn tile_index(&self) -> usize {
        (self.0 & 0x3FF) as usize
    }

    fn palette_index(&self) -> usize {
        ((self.0 >> 12) & 0x0F) as usize
    }

    fn x_mirror(&self) -> bool {
        (self.0 >> 10) & 0x01 == 0x01
    }

    fn y_mirror(&self) -> bool {
        (self.0 >> 11) & 0x01 == 0x01
    }
}

#[derive(StructOpt)]
#[structopt(name = "intro_gfx_viewer")]
struct Opt {
    #[structopt(short = "r", long = "rom")]
    rom_path: String,

    #[structopt(short = "d", long = "data")]
    data_offset: HexOffset,

    #[structopt(short = "p", long = "palette")]
    palette_offset: HexOffset,

    #[structopt(short = "m", long = "map")]
    map_offset: HexOffset,
}

fn draw_tile(buffer: &mut [u32], buffer_width: usize, buffer_height: usize, tile: Tile, tile_x: usize, tile_y: usize, gfx_data: &[u8], gfx_palette: &[Rgb555]) {
    for pixel_y in 0..8 {
        for pixel_x in 0..8 {
            let palette_index = {
                let pixel_x_mirror = if tile.x_mirror() { 7 - pixel_x } else { pixel_x };
                let pixel_y_mirror = if tile.y_mirror() { 7 - pixel_y } else { pixel_y };
                let value = gfx_data[tile.tile_index() * 32 + pixel_y_mirror * 4 + pixel_x_mirror / 2];

                if (pixel_x_mirror & 0x01) == 0 {
                    value & 0x0F
                } else {
                    (value >> 4)
                }
            };

            let x_pos = tile_x * 8 + pixel_x;
            let y_pos = tile_y * 8 + pixel_y;
            buffer[y_pos * buffer_width + x_pos] = gfx_palette[tile.palette_index() * 16 + palette_index as usize].argb8888();
        }
    }
}

// cargo run --package intro_gfx_viewer -- --data 0828455C --map 082856DE --palette 082844FC --rom rom/wario_land_4.gba
// cargo run --package intro_gfx_viewer -- --data 08283f54 --map 082844ee --palette 08283f14 --rom rom/wario_land_4.gba
// cargo run --package intro_gfx_viewer -- --data 0828602c --map 0828759c --palette 08285fcc --rom rom/wario_land_4.gba

fn main() {
    let opt = Opt::from_args();

    let rom_data = {
        let mut result: Vec<u8> = Vec::new();
        let mut file = File::open(opt.rom_path).unwrap();
        file.read_to_end(&mut result).unwrap();
        result
    };

    let gfx_data = decompress_lz77(&rom_data[(opt.data_offset.0 as usize)..]).unwrap();

    let gfx_palette = {
        let mut palette: Vec<Rgb555> = Vec::new();
        let mut cursor = Cursor::new(&rom_data[opt.palette_offset.0 as usize..]);

        for _ in 0..256 {
            palette.push(Rgb555(cursor.read_u16::<LittleEndian>().unwrap()))
        }

        palette
    };


    let (gfx_map_width, gfx_map_height) = (32, 32);

    let gfx_map = {
        let mut cursor = Cursor::new(&rom_data[opt.map_offset.0 as usize..]);
        let mut map: Vec<Tile> = vec![Tile(0); gfx_map_width * gfx_map_height];

        loop {
            let header = cursor.read_u16::<LittleEndian>().unwrap();
            if header == 0 {
                break;
            }

            let start_offset = (((header as u32) << 0x11) >> 0x16) as usize;
            let length = (header & 0x1F) as usize + 1;
            let value = cursor.read_u16::<LittleEndian>().unwrap();

            if (header & 0x8000) == 0 {
                for i in 0..length {
                    map[start_offset + i] = Tile(value + i as u16);
                }
            } else {
                for i in 0..length {
                    map[start_offset + i] = Tile(value);
                }
            }
        }

        map
    };


    let buffer_width = gfx_map_width * 8;
    let buffer_height = gfx_map_height * 8;

    let mut buffer: Vec<u32> = vec![0; buffer_width * buffer_height]; //0xAARRGGBB

    for tile_y in 0..gfx_map_height {
        for tile_x in 0..gfx_map_width {
            let tile = gfx_map[tile_y * gfx_map_width + tile_x];

            draw_tile(&mut buffer[..], buffer_width, buffer_height, tile, tile_x, tile_y, &gfx_data[..], &gfx_palette[..]);
        }
    }

    {
        let window_options = WindowOptions {
            borderless: false,
            title: true,
            resize: false,
            scale: Scale::X2,
        };
        let mut window = Window::new("intro_gfx_viewer", buffer_width, buffer_height, window_options).unwrap();

        while window.is_open() && !window.is_key_down(Key::Escape) {
            window.update_with_buffer(&buffer).unwrap();
        }
    }
}
