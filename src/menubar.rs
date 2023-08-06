use crate::Menu;

// ----------------------------------------------------------------------------

pub struct MenuBar {
    pub(crate) main_menu: Option<Menu>,
    pub(crate) menus: Vec<Menu>,
}

impl MenuBar {
    pub fn new(menus: Vec<Menu>) -> Self {
        Self {
            main_menu: None, // Keep the default
            menus,
        }
    }

    pub fn overwrite_main_menu(self, main_menu: Menu) -> Self {
        Self {
            main_menu: Some(main_menu),
            ..self
        }
    }
}
