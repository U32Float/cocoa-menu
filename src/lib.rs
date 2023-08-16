use icrate::AppKit::{NSApplication, NSMenuItem};
use objc2::rc::autoreleasepool;
use objc2::runtime;

#[cfg(target_os = "macos")]
mod item;
pub use item::{Action, MenuItem};
#[cfg(target_os = "macos")]
mod shortcut;
pub use shortcut::{Key, Shortcut};
#[cfg(target_os = "macos")]
mod menu;
pub use menu::Menu;
#[cfg(target_os = "macos")]
mod menubar;
pub use menubar::MenuBar;
#[cfg(target_os = "macos")]
mod image;
pub use image::Image;

// ----------------------------------------------------------------------------

#[allow(non_camel_case_types)]
pub(crate) type id = *mut runtime::NSObject;

#[allow(non_upper_case_globals)]
pub(crate) const nil: id = 0 as id;

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
    unsafe {
        autoreleasepool(|_| {
            let app = NSApplication::sharedApplication();
            let main_menu = app.mainMenu().unwrap();
            let num_items = main_menu.numberOfItems();

            if menubar.main_menu.is_some() {
                main_menu.removeAllItems();

                let item = NSMenuItem::new();
                item.setSubmenu(Some(&menubar.main_menu.as_ref().unwrap().to_objc()));
                main_menu.addItem(&item);
            } else {
                // Remove all items except the main menu
                for i in 1..num_items {
                    main_menu.removeItemAtIndex(i);
                }
            }

            for menu in menubar.menus.iter() {
                let item = NSMenuItem::new();
                item.setSubmenu(Some(&menu.to_objc()));
                main_menu.addItem(&item);
            }
        });
    }
}
