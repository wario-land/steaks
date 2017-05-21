#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate byteorder;
extern crate bincode;

mod project;

use std::fs::File;
use std::io::{Read, Write};
use project::import;
use bincode::{serialize, Infinite};

fn main() {
    println!("Steaks!");
    println!("Eat as much as you can!!");

    let mut rom_data: Vec<u8> = Vec::new();
    let mut rom_file = File::open("/home/tibor/git/gba/gba-tools/rom/wario_land_4.gba").unwrap();
    rom_file.read_to_end(&mut rom_data).unwrap();

    let project = import(&rom_data[..]).unwrap();
    let encoded: Vec<u8> = serialize(&project, Infinite).unwrap();
    let mut project_file = File::create("project.steaks").unwrap();
    project_file.write_all(&encoded).unwrap();
}
