use objc::runtime::Object;
use objc::{class, sel_impl};
use objc::{
    msg_send,
    runtime::{self},
};

#[macro_use(sel)]
extern crate objc;

#[cfg(target_os = "macos")]
mod item;
pub use item::{Action, MenuItem};
mod shortcut;
pub use shortcut::{Key, Shortcut};
#[cfg(target_os = "macos")]
mod menu;
pub use menu::Menu;
#[cfg(target_os = "macos")]
mod menubar;
pub use menubar::MenuBar;

// ----------------------------------------------------------------------------

#[allow(non_camel_case_types)]
pub(crate) type id = *mut runtime::Object;

#[allow(non_upper_case_globals)]
pub(crate) const nil: id = 0 as id;

/// Platform-specific.
#[cfg(target_pointer_width = "32")]
pub(crate) type NSInteger = libc::c_int;

/// Platform-specific.
#[cfg(target_pointer_width = "32")]
pub(crate) type NSUInteger = libc::c_uint;

/// Platform-specific.
#[cfg(target_pointer_width = "64")]
pub(crate) type NSInteger = libc::c_long;

/// Platform-specific.
#[cfg(target_pointer_width = "64")]
pub(crate) type NSUInteger = libc::c_ulong;

#[derive(Debug)]
#[repr(C)]
pub(crate) struct CGPoint {
    pub x: f64,
    pub y: f64,
}

pub(crate) unsafe fn autorelease<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let autorelease_pool: *mut Object = msg_send![class!(NSAutoreleasePool), new];
    let ret = f();
    let _: () = msg_send![autorelease_pool, release];
    ret
}

/// Activates a `MenuBar`.
///
/// # Info
/// Everytime this method is called, all items are removed from the application menubar and
/// a new objective-c instance of `MenuBar` is created and added to the application menubar.
///
/// # Warning
/// - This will panic if no shared application exists.
/// - Does nothing if called before application is done initializing.
#[cfg(target_os = "macos")]
pub fn activate_menubar(menubar: &MenuBar) {
    let app_cls = class!(NSApplication);
    let item_cls = class!(NSMenuItem);

    unsafe {
        autorelease(|| {
            let app: id = msg_send![app_cls, sharedApplication];

            let main_menu: id = msg_send![app, mainMenu];

            let num_items: NSInteger = msg_send![main_menu, numberOfItems];

            if menubar.main_menu.is_some() {
                let _: () = msg_send![main_menu, removeAllItems];

                let item: id = msg_send![item_cls, new];
                let item: id = msg_send![item, autorelease]; // Prevent memory leak
                let _: () =
                    msg_send![item, setSubmenu:&*menubar.main_menu.as_ref().unwrap().to_objc()];
                let _: () = msg_send![main_menu, addItem: item];
            } else {
                // Remove all items except the main menu
                for i in 1..num_items {
                    let _: () = msg_send![main_menu, removeItemAtIndex: i];
                }
            }

            for menu in menubar.menus.iter() {
                let item: id = msg_send![item_cls, new];
                let item: id = msg_send![item, autorelease]; // Prevent memory leak
                let _: () = msg_send![item, setSubmenu:&*menu.to_objc()];
                let _: () = msg_send![main_menu, addItem: item];
            }
        });
    }
}
