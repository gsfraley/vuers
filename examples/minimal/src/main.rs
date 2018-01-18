#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate vuers;

use vuers::Vue;

fn main() {
    stdweb::initialize();

    Vue::builder("#app")
        .with_data(js_obj! { message: "Hello World!" })
        .finish();
}