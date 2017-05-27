#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate itertools;
#[macro_use]
extern crate serde_derive;
extern crate num;
extern crate serde;
extern crate uuid;
extern crate byteorder;

#[macro_use]
mod rom_map;
mod project;
mod import;


use import::import;
use rom_map::Rom;

fn main() {
    let rom = Rom::new("/home/tibor/git/gba/gba-tools/rom/wario_land_4.gba").unwrap();
    let project = import(&rom).unwrap();
    println!("{:#?}", project);
}
