use objc::runtime::Object;
use objc_id::Id;

use crate::item::MenuItem;

// ----------------------------------------------------------------------------

pub struct Menu {
    name: String,
    items: Vec<MenuItem>,
}

impl Menu {
    pub fn new(name: &str, items: Vec<MenuItem>) -> Self {
        Self {
            name: name.to_string(),
            items,
        }
    }

    pub fn push(&mut self, item: MenuItem) {
        self.items.push(item);
    }

    pub fn append(&mut self, mut items: Vec<MenuItem>) {
        self.items.append(&mut items);
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
}

impl Menu {
    fn to_objc(&self) -> Id<Object> {
        todo!()
    }
}
