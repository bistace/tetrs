#![allow(dead_code)]
#![warn(clippy::all, clippy::pedantic)]

use crate::interface::Interface;

mod engine;
mod interface;

fn main() {
    let interface = Interface::new();
    interface.run();
}
