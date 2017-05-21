#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate byteorder;
extern crate serde_json;

mod project;
mod compiler;
mod decompiler;

use std::fs::File;
use std::io::Read;
use decompiler::decompile;

fn main() {
    println!("Steaks!");
    println!("Eat as much as you can!!");

    let mut rom_data: Vec<u8> = Vec::new();
    let mut rom_file = File::open("/home/tibor/git/gba/gba-tools/rom/wario_land_4.gba").unwrap();
    rom_file.read_to_end(&mut rom_data).unwrap();

    let project = decompile(&rom_data[..]).unwrap();
    println!("{}", serde_json::to_string_pretty(&project).unwrap());
}
