#[macro_use]
extern crate stdweb;

use stdweb::Value;

pub struct Vue {
    pub obj: Value
}

impl Vue {
    pub fn new(el: &str) -> Self {
        Vue {
            obj: js! {
                new Vue({
                    el: @{el},
                    data: {
                        message: "Hello"
                    }
                })
            }
        }
    }
}