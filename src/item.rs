use objc::runtime::Object;
use objc_id::Id;

// ----------------------------------------------------------------------------

pub enum MenuItem {}

impl MenuItem {
    fn to_objc(&self) -> Id<Object> {
        todo!()
    }
}
