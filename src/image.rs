use objc::runtime::Object;
use objc::sel_impl;
use objc::{class, msg_send};
use objc_id::{Id, Shared};

use crate::id;

// ----------------------------------------------------------------------------

#[derive(Clone)]
pub struct Image(pub(crate) Id<Object, Shared>);

impl Image {
    pub fn from_raw(bytes: &[u8]) -> Self {
        let cls = class!(NSData);
        let data: id = unsafe {
            let obj: id = msg_send![cls, alloc];
            let obj: id = msg_send![obj, initWithBytes:bytes.as_ptr()
                                                       length:bytes.len()];
            obj
        };
        let obj: Id<Object, Shared> = unsafe {
            let alloc: id = msg_send![class!(NSImage), alloc];
            let obj = Id::from_retained_ptr(msg_send![alloc, initWithData: data]);
            let _: () = msg_send![data, release];
            obj
        };

        Self(obj)
    }
}
