#[macro_use]
extern crate stdweb;

use stdweb::{Object, Value};

pub struct Vue {
    instance: Object
}

impl Vue {
    pub fn new(options: Object) -> Self {
        Vue {
            instance: js! { return new Vue( @{options} ) }
                .into_object().unwrap()
        }
    }
}

#[macro_export]
macro_rules! vue {
    ( $($x:tt)* ) => {
        Vue::new(
            js! { return { $($x)* } }
                .into_object().unwrap()
        )
    }
}