use objc::{class, sel_impl};
use objc::{msg_send, runtime::Object};
use objc_foundation::{INSString, NSString};
use objc_id::Id;

use crate::{id, Menu};

// ----------------------------------------------------------------------------

#[derive(Clone)]
pub enum MenuItem {
    Dummy(String),

    SubMenu(Menu),
}

impl MenuItem {
    pub(crate) fn to_objc(&self) -> Id<Object> {
        let item_cls = class!(NSMenuItem);

        match self {
            MenuItem::Dummy(title) => unsafe {
                let alloc: id = msg_send![item_cls, alloc];
                let title = NSString::from_str(title);
                let key = NSString::from_str("");
                let item: id = msg_send![alloc, initWithTitle:&*title action:sel!(terminate:) keyEquivalent:&*key];
                Id::from_retained_ptr(item)
            },
            MenuItem::SubMenu(menu) => unsafe {
                let item: id = msg_send![item_cls, new];
                let _: () = msg_send![item, setSubmenu:&*menu.to_objc()];
                Id::from_retained_ptr(item)
            },
        }
    }
}
