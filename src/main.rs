#[macro_use] extern crate enum_primitive;
#[macro_use] extern crate itertools;
#[macro_use] extern crate serde_derive;
extern crate num;
extern crate serde;
extern crate uuid;

#[macro_use]
mod rom_map;
mod project;
mod import;

use std::io::Read;
use std::fs::File;
use import::import;

fn main() {
    let mut rom_data: Vec<u8> = Vec::new();
    let mut rom_file = File::open("/home/tibor/git/gba/gba-tools/rom/wario_land_4.gba").unwrap();
    rom_file.read_to_end(&mut rom_data).unwrap();

    let project = import(&rom_data).unwrap();
    println!("{:#?}", project);
}
