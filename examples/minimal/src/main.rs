#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate vuers;

use vuers::Vue;

fn main() {
    stdweb::initialize();

    vue! {
        el: "#app",
        data: {
            message: "Hello world!"
        }
    };
}