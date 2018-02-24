#[macro_use]
extern crate stdweb;

use std::collections::HashMap;

use stdweb::{Object, Value};

pub struct VueBuilder<'a> {
    el: Option<&'a str>,
    data: Option<HashMap<&'a str, Value>>
}

impl<'a> VueBuilder<'a> {
    pub fn new() -> Self {
        VueBuilder {
            el: None,
            data: None
        }
    }

    pub fn with_el(mut self, el: &'a str) -> Self {
        self.el = Some(el);
        self
    }

    pub fn with_data(mut self, data: HashMap<&'a str, Value>) -> Self {
        self.data = Some(data);
        self
    }

    pub fn build(self) -> Vue {
        Vue::new(
            self.el.unwrap(),
            self.data.unwrap()
        )
    }
}

pub struct Vue {
    instance: Object
}

impl Vue {
    pub fn new<'a>(el: &'a str, data: HashMap<&'a str, Value>) -> Self {
        let data_obj: Object = data.into();
        
        Vue {
            instance: js! {
                return new Vue({
                    el: @{el},
                    data: @{data_obj}
                })
            }.into_object().unwrap()
        }
    }

    pub fn builder<'a>() -> VueBuilder<'a> {
        VueBuilder::new()
    }

    pub fn instance(&self) -> &Object {
        &self.instance
    }
}