use std::{ptr::NonNull, sync::Arc};

use icrate::{AppKit::NSImage, Foundation::NSData};
use libc::c_void;
use objc2::{rc::Id, ClassType};

// ----------------------------------------------------------------------------

#[derive(Clone)]
pub enum Image {
    Static(&'static [u8]),
    Objc(Id<NSImage>),
    CopyOnPass(Arc<Vec<u8>>),
}

impl Image {
    /// Image data is never copied.
    pub fn from_bytes_no_copy(bytes: &'static [u8]) -> Self {
        Self::Static(bytes)
    }

    /// Image data is shared between copies.
    ///
    /// # Warning
    /// This should be used inside an objective-c autorelease pool, otherwise it could leak memory.
    ///
    /// If not, use `Self::from_bytes_copy_on_pass(..)` instead.
    pub unsafe fn from_bytes(bytes: &[u8]) -> Self {
        let data = NSData::with_bytes(bytes);

        let obj = unsafe {
            let alloc = NSImage::alloc();
            NSImage::initWithData(alloc, &data)
        }
        .unwrap();

        Self::Objc(obj)
    }

    /// The image data is shared between each copy. But on passing `Self` to a `Menu` the data does get
    /// copied however.
    ///
    /// Unlike `Self::from_bytes(..)` there's no need for an autorelease pool.
    pub fn from_bytes_copy_on_pass(bytes: &[u8]) -> Self {
        Self::CopyOnPass(Arc::new(bytes.to_vec()))
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
            Image::Objc(obj) => obj.clone(),
            Image::CopyOnPass(bytes) => unsafe {
                let data = NSData::with_bytes(&bytes);
                let alloc = NSImage::alloc();
                NSImage::initWithData(alloc, &data).unwrap()
            },
        }
    }
}
