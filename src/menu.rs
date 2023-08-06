use objc::sel_impl;
use objc::{class, msg_send, runtime::Object};
use objc_foundation::{INSString, NSString};
use objc_id::Id;

use crate::id;
use crate::item::MenuItem;

// ----------------------------------------------------------------------------

#[derive(Clone)]
pub struct Menu {
    title: String,
    items: Vec<MenuItem>,
}

impl Menu {
    pub fn new(title: &str, items: Vec<MenuItem>) -> Self {
        Self {
            title: title.to_string(),
            items,
        }
    }

    pub fn add_item(&mut self, item: &MenuItem) {
        self.items.push(item.clone());
    }

    pub fn append_items(&mut self, items: &[MenuItem]) {
        for item in items {
            self.add_item(item);
        }
    }

    pub fn to_objc(&self) -> Id<Object> {
        let menu_cls = class!(NSMenu);

        let menu: id = unsafe {
            let alloc: id = msg_send![menu_cls, alloc];
            let title = NSString::from_str(&self.title);
            msg_send![alloc, initWithTitle:&*title]
        };

        for item in self.items.iter() {
            let objc = item.to_objc();
            let _: () = unsafe { msg_send![menu, addItem:&*objc] };
        }

        unsafe { Id::from_retained_ptr(menu) }
    }
}
