use objc::sel_impl;
use objc::{
    class, msg_send,
    runtime::{self, Class, Object},
};

#[macro_use(sel)]
extern crate objc;

mod item;
mod menu;

// ----------------------------------------------------------------------------

#[allow(non_camel_case_types)]
pub type id = *mut runtime::Object;
