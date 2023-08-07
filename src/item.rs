use objc::runtime::{NO, YES};
use objc::{class, sel_impl};
use objc::{msg_send, runtime::Object};
use objc_foundation::{INSString, NSString};
use objc_id::Id;

use crate::{id, Menu};

// ----------------------------------------------------------------------------

#[derive(Clone)]
enum MenuItemType {
    Dummy(String),

    SubMenu(Menu),

    Separator,
}

impl MenuItemType {
    fn to_objc(&self) -> Id<Object> {
        let item_cls = class!(NSMenuItem);

        match self {
            MenuItemType::Dummy(title) => unsafe {
                let alloc: id = msg_send![item_cls, alloc];
                let title = NSString::from_str(title);
                let key = NSString::from_str("");
                let item: id = msg_send![alloc, initWithTitle:&*title action: sel!(fireBlockAction:) keyEquivalent:&*key];
                let item: id = msg_send![item, autorelease];
                Id::from_ptr(item)
            },
            MenuItemType::SubMenu(menu) => unsafe {
                let item: id = msg_send![item_cls, new];
                let item: id = msg_send![item, autorelease];
                let _: () = msg_send![item, setSubmenu:&*menu.to_objc()];
                Id::from_ptr(item)
            },
            MenuItemType::Separator => unsafe {
                let item: id = msg_send![item_cls, separatorItem];
                Id::from_ptr(item)
            },
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Clone)]
pub struct MenuItem {
    item_type: MenuItemType,
    enabled: bool,
    hidden: bool,
}

impl Default for MenuItem {
    fn default() -> Self {
        Self {
            item_type: MenuItemType::Dummy("default".into()),
            enabled: true,
            hidden: false,
        }
    }
}

impl MenuItem {
    pub const SEPARATOR: Self = MenuItem {
        item_type: MenuItemType::Separator,
        enabled: true,
        hidden: false,
    };

    pub const fn dummy(title: String) -> Self {
        Self {
            item_type: MenuItemType::Dummy(title),
            enabled: true,
            hidden: false,
        }
    }

    pub const fn sub_menu(menu: Menu) -> Self {
        Self {
            item_type: MenuItemType::SubMenu(menu),
            enabled: true,
            hidden: false,
        }
    }

    pub fn enabled(self, enabled: bool) -> Self {
        Self { enabled, ..self }
    }

    pub fn hidden(self, hidden: bool) -> Self {
        Self { hidden, ..self }
    }

    pub(crate) fn to_objc(&self) -> Id<Object> {
        let item = self.item_type.to_objc();

        unsafe {
            if !self.enabled {
                let _: () = msg_send![item, setEnabled: NO];
            }
            if self.hidden {
                let _: () = msg_send![item, setHidden: YES];
            }
        }

        item
    }
}
