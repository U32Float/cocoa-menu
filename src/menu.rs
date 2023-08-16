use icrate::AppKit::{NSEvent, NSMenu};
use icrate::Foundation::{CGFloat, CGPoint, NSString};
use objc2::rc::{autoreleasepool, Id};
use objc2::ClassType;

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

    /// Shows a popup of this menu at the current mouse position.
    pub fn show_popup(&self) {
        let pos = unsafe { NSEvent::mouseLocation() };
        self.show_popup_at([pos.x as u32, pos.y as u32]);
    }

    /// Shows a popup of this menu at the given position.
    pub fn show_popup_at(&self, position: [u32; 2]) {
        let point = CGPoint::new(position[0] as CGFloat, position[1] as CGFloat);
        unsafe {
            autoreleasepool(|_| {
                let menu = self.to_objc();
                menu.popUpMenuPositioningItem_atLocation_inView(None, point, None);
            });
        }
    }

    pub(crate) fn to_objc(&self) -> Id<NSMenu> {
        unsafe {
            let alloc = NSMenu::alloc();
            let title = NSString::from_str(&self.title);
            let menu = NSMenu::initWithTitle(alloc, &title);
            menu.setAutoenablesItems(false);

            for item in self.items.iter() {
                menu.addItem(&item.to_objc());
            }

            menu
        }
    }
}
