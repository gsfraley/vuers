extern crate stdweb;
extern crate vuers;

use vuers::Vue;

fn main() {
    stdweb::initialize();
    Vue::new("#app");
}