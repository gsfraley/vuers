extern crate stdweb;
extern crate vuers;

use std::collections::HashMap;

use stdweb::Value;

use vuers::Vue;

fn main() {
    stdweb::initialize();

    let mut data: HashMap<&str, Value> = HashMap::new();
    data.insert("message", "Hello World!".into());

    Vue::builder()
        .with_el("#app")
        .with_data(data)
        .build();
}