use objc::{class, sel_impl};
use objc::{
    msg_send,
    runtime::{self, Class},
};

#[macro_use(sel)]
extern crate objc;

mod item;
pub use item::MenuItem;
mod menu;
pub use menu::Menu;
mod menubar;
pub use menubar::MenuBar;

// ----------------------------------------------------------------------------

#[allow(non_camel_case_types)]
pub type id = *mut runtime::Object;

#[allow(non_upper_case_globals)]
pub const nil: id = 0 as id;

/// Platform-specific.
#[cfg(target_pointer_width = "64")]
pub type NSInteger = libc::c_long;

/// Activates a `MenuBar`.
///
/// # Info
/// Everytime this method is called, all items are removed from the application menubar and
/// a new objective-c instance of `MenuBar` is created and added to the application menubar.
///
/// # Warning
/// - This will panic if no shared application exists.
/// - If called before application is finished intializing, it will do nothing.
pub fn activate_menubar(menubar: &MenuBar) {
    let cls = Class::get("NSApplication").unwrap();
    let item_cls = class!(NSMenuItem);

    unsafe {
        let app: id = msg_send![cls, sharedApplication];

        let main_menu: id = msg_send![app, mainMenu];

        let num_items: NSInteger = msg_send![main_menu, numberOfItems];

        if menubar.main_menu.is_some() {
            let _: () = msg_send![main_menu, removeAllItems];

            let item: id = msg_send![item_cls, new];
            let _: () = msg_send![item, setSubmenu:&*menubar.main_menu.as_ref().unwrap().to_objc()];
            let _: () = msg_send![main_menu, addItem: item];
        } else {
            // Remove all items except the main menu
            for i in 1..num_items {
                let _: () = msg_send![main_menu, removeItemAtIndex: i];
            }
        }

        for menu in menubar.menus.iter() {
            let item: id = msg_send![item_cls, new];
            let _: () = msg_send![item, setSubmenu:&*menu.to_objc()];
            let _: () = msg_send![main_menu, addItem: item];
        }
    }
}
