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
extern crate gba_rs;
#[macro_use]
extern crate nom;

#[macro_use]
mod rom_map;
mod project;
mod import;

use import::import;

fn main() {
    let project = import("/home/tibor/git/gba/gba-tools/rom/wario_land_4.gba").unwrap();
    println!("{:#?}", project);
}
