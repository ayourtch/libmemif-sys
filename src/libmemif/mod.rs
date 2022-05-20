#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_variables, unused_mut)]
#![allow(unused)]

// use std;
use std::fmt::{Debug, Error, Formatter};

/*
 * #[repr(C)]
pub struct vnet_sw_interface_t {
    pub _bindgen_opaque_blob: [u32; 10usize],
}
*/

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
