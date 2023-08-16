use std::ptr::NonNull;

use icrate::{AppKit::NSImage, Foundation::NSData};
use libc::c_void;
use objc2::{rc::Id, ClassType};

// ----------------------------------------------------------------------------

#[derive(Clone)]
pub enum Image {
    Static(&'static [u8]),
    Unsafe(Id<NSImage>),
}

impl Image {
    pub fn from_bytes_no_copy(bytes: &'static [u8]) -> Self {
        Self::Static(bytes)
    }

    /// # Warning
    /// If this is not used inside an objective-c autorelease pool, it could leak memory.
    pub unsafe fn from_bytes_copy(bytes: &[u8]) -> Self {
        let data = NSData::with_bytes(bytes);

        let obj = unsafe {
            let alloc = NSImage::alloc();
            NSImage::initWithData(alloc, &data)
        }
        .unwrap();

        Self::Unsafe(obj)
    }

    pub(crate) fn to_objc(&self) -> Id<NSImage> {
        match self {
            Image::Static(bytes) => unsafe {
                let ptr = *bytes as *const [u8];
                let ptr: NonNull<c_void> = NonNull::new_unchecked(ptr as *mut c_void);
                let alloc = NSData::alloc();
                let data =
                    NSData::initWithBytesNoCopy_length_freeWhenDone(alloc, ptr, bytes.len(), false);

                let alloc = NSImage::alloc();
                NSImage::initWithData(alloc, &data).unwrap()
            },
            Image::Unsafe(obj) => obj.clone(),
        }
    }
}
