#[macro_use]
extern crate stdweb;

use std::collections::HashMap;

use stdweb::{Object, Value};

pub struct VueBuilder<'a> {
    el: &'a str,
    data: Option<Object>
}

impl<'a> VueBuilder<'a> {
    pub fn new(el: &'a str) -> Self {
        VueBuilder {
            el: el,
            data: None
        }
    }

    pub fn with_data(mut self, data: Object) -> Self {
        self.data = Some(data);
        self
    }

    pub fn finish(self) -> Vue {
        let mut options_map: HashMap<&str, Value> = HashMap::new();
        options_map.insert("el", self.el.into());
        options_map.insert("data", Value::Object(self.data.unwrap()));

        let options_obj = Object::from(options_map);
        Vue::new(options_obj)
    }
}

pub struct Vue {
    instance: Object
}

impl Vue {
    pub fn new(options: Object) -> Self {
        Vue {
            instance: js! {
                return new Vue( @{options} )
            }.into_object().unwrap()
        }
    }

    pub fn builder<'a>(el: &'a str) -> VueBuilder<'a> {
        VueBuilder::new(el)
    }

    pub fn instance(&self) -> &Object {
        &self.instance
    }
}

#[macro_export]
macro_rules! js_obj {
    ( $($x:tt)* ) => {
        js! { return { $($x)* } }
            .into_object().unwrap()
    }
}