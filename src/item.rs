use std::sync::Arc;

use icrate::{AppKit::NSMenuItem, Foundation::NSString};
use objc2::{
    class,
    declare::ClassBuilder,
    msg_send,
    rc::Id,
    runtime::{AnyClass, NSObject, Sel},
    sel, ClassType,
};
use once_cell::sync::Lazy;

use crate::{id, nil, Image, Menu, Shortcut};

// ----------------------------------------------------------------------------
#[derive(Clone)]
pub struct MenuItem {
    item_type: MenuItemType,

    enabled: bool,
    hidden: bool,

    image: Option<Image>,
    image_on: Option<Image>,
    image_off: Option<Image>,
    image_mixed: Option<Image>,
}

impl Default for MenuItem {
    fn default() -> Self {
        Self {
            item_type: MenuItemType::Dummy("default".into()),
            enabled: true,
            hidden: false,
            image: None,
            image_on: None,
            image_off: None,
            image_mixed: None,
        }
    }
}

impl MenuItem {
    pub const SEPARATOR: Self = MenuItem {
        item_type: MenuItemType::Separator,
        enabled: true,
        hidden: false,
        image: None,
        image_on: None,
        image_off: None,
        image_mixed: None,
    };

    pub const fn dummy(title: String) -> Self {
        Self {
            item_type: MenuItemType::Dummy(title),
            enabled: true,
            hidden: false,
            image: None,
            image_on: None,
            image_off: None,
            image_mixed: None,
        }
    }

    pub const fn sub_menu(menu: Menu) -> Self {
        Self {
            item_type: MenuItemType::SubMenu(menu),
            enabled: true,
            hidden: false,
            image: None,
            image_on: None,
            image_off: None,
            image_mixed: None,
        }
    }

    pub const fn button(title: String, action: Action, shortcut: Option<Shortcut>) -> Self {
        Self {
            item_type: MenuItemType::Button(title, action, shortcut),
            enabled: true,
            hidden: false,
            image: None,
            image_on: None,
            image_off: None,
            image_mixed: None,
        }
    }
    pub fn enabled(self, enabled: bool) -> Self {
        Self { enabled, ..self }
    }

    pub fn hidden(self, hidden: bool) -> Self {
        Self { hidden, ..self }
    }

    pub fn image(self, image: Option<Image>) -> Self {
        Self { image, ..self }
    }

    pub fn image_on(self, image_on: Option<Image>) -> Self {
        Self { image_on, ..self }
    }

    pub fn image_off(self, image_off: Option<Image>) -> Self {
        Self { image_off, ..self }
    }

    pub fn image_mixed(self, image_mixed: Option<Image>) -> Self {
        Self {
            image_mixed,
            ..self
        }
    }

    pub(crate) fn to_objc(&self) -> Id<NSMenuItem> {
        let item = self.item_type.to_objc();

        unsafe {
            if !self.enabled {
                item.setEnabled(false);
            }
            if self.hidden {
                item.setHidden(true);
            }
            if let Some(ref img) = self.image {
                item.setImage(Some(&img.to_objc()));
            }
            if let Some(ref img_on) = self.image_on {
                item.setOnStateImage(Some(&img_on.to_objc()));
            }
            if let Some(ref img_off) = self.image_off {
                item.setOffStateImage(Some(&img_off.to_objc()));
            }
            if let Some(ref img_mixed) = self.image_mixed {
                item.setMixedStateImage(Some(&img_mixed.to_objc()));
            }
        }

        item
    }
}

// ----------------------------------------------------------------------------

#[derive(Clone)]
enum MenuItemType {
    Dummy(String),

    Button(String, Action, Option<Shortcut>),

    SubMenu(Menu),

    Separator,
}

impl MenuItemType {
    fn to_objc(&self) -> Id<NSMenuItem> {
        match self {
            MenuItemType::Dummy(title) => unsafe {
                let title = NSString::from_str(title);
                let key = NSString::from_str("");
                let alloc = NSMenuItem::alloc();
                NSMenuItem::initWithTitle_action_keyEquivalent(alloc, &title, None, &key)
            },
            MenuItemType::SubMenu(menu) => unsafe {
                let item = NSMenuItem::new();
                item.setSubmenu(Some(&menu.to_objc()));
                item
            },
            MenuItemType::Separator => unsafe { NSMenuItem::separatorItem() },
            MenuItemType::Button(title, action, shortcut) => unsafe {
                let alloc: id = msg_send![register_menu_item_class(), alloc];

                let title = NSString::from_str(title);
                let key = if let Some(shortcut) = shortcut {
                    NSString::from_str(&shortcut.key.to_string())
                } else {
                    NSString::from_str("")
                };
                let item: id = msg_send![alloc, initWithTitle:&*title action: action.to_sel() keyEquivalent:&*key];
                if let Some(shortcut) = shortcut {
                    let _: () = msg_send![item, setKeyEquivalentModifierMask: shortcut.mask()];
                }

                if let Action::Callback(action) = action {
                    register_callback(item, action.clone());
                }

                Id::new(item as *mut NSMenuItem).unwrap()
            },
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Clone)]
pub enum Action {
    Hide,
    HideOthers,
    ShowAll,
    CloseWindow,
    Quit,
    ToggleFullScreen,
    Minimize,
    None,
    Callback(Arc<dyn Fn() + Send + Sync + 'static>),
}

impl Action {
    pub fn callback<F>(f: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        Self::Callback(Arc::new(f))
    }

    fn to_sel(&self) -> Sel {
        match self {
            Action::Hide => sel!(hide:),
            Action::HideOthers => sel!(hideOtherApplications:),
            Action::ShowAll => sel!(unhideAllApplications:),
            Action::CloseWindow => sel!(performClose:),
            Action::Quit => sel!(terminate:),
            Action::ToggleFullScreen => sel!(toggleFullScreen:),
            Action::Minimize => sel!(performMiniaturize:),
            Action::Callback(_) => sel!(fireBlockAction:),
            Action::None => sel!(fireBlockAction:),
        }
    }
}

// ----------------------------------------------------------------------------

static BLOCK_PTR: &'static str = "cacaoMenuItemBlockPtr";

fn register_callback(objc: id, action: Arc<dyn Fn() + 'static>) {
    let handler = Box::new(action);
    let ptr = Box::into_raw(handler);

    unsafe {
        (&mut *objc).set_ivar(BLOCK_PTR, ptr as usize);
        let _: () = msg_send![&*objc, setTarget:&*objc];
    }
}

extern "C" fn dealloc_cacao_menuitem(this: &NSObject, _: Sel) {
    unsafe {
        let ptr: usize = *this.ivar(BLOCK_PTR);
        let obj = ptr as *mut Arc<dyn Fn() + 'static>;

        if !obj.is_null() {
            let _handler = Box::from_raw(obj);
        }

        let _: () = msg_send![this, setTarget:nil];

        let _: () = msg_send![super(this, class!(NSMenuItem)), dealloc];
    }
}

extern "C" fn fire_block_action(this: &NSObject, _: Sel, _item: id) {
    let action = load::<Arc<dyn Fn() + 'static>>(this, BLOCK_PTR);
    action();
}

pub(crate) fn register_menu_item_class() -> &'static AnyClass {
    static CLASS: Lazy<&'static AnyClass> = Lazy::new(|| unsafe {
        let mut builder = ClassBuilder::new("CacaoMenuItem", class!(NSMenuItem)).unwrap();

        builder.add_ivar::<usize>(BLOCK_PTR);

        builder.add_method(
            sel!(dealloc),
            dealloc_cacao_menuitem as unsafe extern "C" fn(_, _),
        );

        builder.add_method(
            sel!(fireBlockAction:),
            fire_block_action as unsafe extern "C" fn(_, _, id),
        );

        builder.register()
    });

    &CLASS
}

pub fn load<'a, T>(this: &'a NSObject, ptr_name: &str) -> &'a T {
    unsafe {
        let ptr: usize = *this.ivar(ptr_name);
        let obj = ptr as *const T;
        &*obj
    }
}
