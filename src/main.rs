#![allow(dead_code)]

use crate::interface::Interface;

mod engine;
mod interface;

fn main() {
    let interface = Interface::new();
    interface.run();
}
