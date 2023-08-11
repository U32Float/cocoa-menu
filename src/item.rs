use std::sync::{Arc, Once};

use objc::declare::ClassDecl;
use objc::runtime::{Class, Sel, NO, YES};
use objc::{class, sel_impl};
use objc::{msg_send, runtime::Object};
use objc_foundation::{INSString, NSString};
use objc_id::Id;

use crate::{id, nil, Menu, Shortcut};

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

    pub const fn button(title: String, action: Action, shortcut: Option<Shortcut>) -> Self {
        Self {
            item_type: MenuItemType::Button(title, action, shortcut),
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

// ----------------------------------------------------------------------------

#[derive(Clone)]
enum MenuItemType {
    Dummy(String),

    Button(String, Action, Option<Shortcut>),

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
                let item: id =
                    msg_send![alloc, initWithTitle:&*title action: sel!(NULL:) keyEquivalent:&*key];
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

                let item: id = msg_send![item, autorelease];
                Id::from_ptr(item)
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

extern "C" fn dealloc_cacao_menuitem(this: &Object, _: Sel) {
    unsafe {
        let ptr: usize = *this.get_ivar(BLOCK_PTR);
        let obj = ptr as *mut Arc<dyn Fn() + 'static>;

        if !obj.is_null() {
            let _handler = Box::from_raw(obj);
        }

        let _: () = msg_send![this, setTarget:nil];

        let _: () = msg_send![super(this, class!(NSMenuItem)), dealloc];
    }
}

extern "C" fn fire_block_action(this: &Object, _: Sel, _item: id) {
    let action = load::<Arc<dyn Fn() + 'static>>(this, BLOCK_PTR);
    action();
}

pub(crate) fn register_menu_item_class() -> *const Class {
    static mut APP_CLASS: *const Class = 0 as *const Class;
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        let superclass = class!(NSMenuItem);
        let mut decl = ClassDecl::new("CacaoMenuItem", superclass).unwrap();
        decl.add_ivar::<usize>(BLOCK_PTR);

        decl.add_method(
            sel!(dealloc),
            dealloc_cacao_menuitem as extern "C" fn(&Object, _),
        );
        decl.add_method(
            sel!(fireBlockAction:),
            fire_block_action as extern "C" fn(&Object, _, id),
        );

        APP_CLASS = decl.register();
    });

    unsafe { APP_CLASS }
}

pub fn load<'a, T>(this: &'a Object, ptr_name: &str) -> &'a T {
    unsafe {
        let ptr: usize = *this.get_ivar(ptr_name);
        let obj = ptr as *const T;
        &*obj
    }
}
