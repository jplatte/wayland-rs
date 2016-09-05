#[macro_use] extern crate wayland_sys;

pub use generated::server as protocol;

use wayland_sys::server::{wl_resource, wl_client};
use wayland_sys::common::{wl_interface, wl_argument};

pub trait Resource {
    /// Pointer to the underlying wayland proxy object
    fn ptr(&self) -> *mut wl_resource;
    /// Create an instance from point a wayland pointer
    ///
    /// The pointer must refer to a valid wayland proxy
    /// of the appropriate interface.
    unsafe fn from_ptr(*mut wl_resource) -> Self;
    /// Pointer to the interface representation
    fn interface_ptr() -> *const wl_interface;
    /// Internal wayland name of this interface
    fn interface_name() -> &'static str;
    /// Max version of this interface supported
    fn supported_version() -> u32;
    /// Current version of the interface this resource is instanciated with
    fn version(&self) -> i32;
}

pub unsafe trait Handler<T: Resource> {
    unsafe fn message(&mut self, evq: &mut EventQueueHandle, client: &Client, proxy: &T, opcode: u32, args: *const wl_argument) -> Result<(),()>;
}


pub struct EventQueueHandle;

pub struct Client {
    ptr: *mut wl_client
}

impl Client {
    pub fn ptr(&self) -> *mut wl_client {
        self.ptr
    }

    pub unsafe fn from_ptr(ptr: *mut wl_client) -> Client {
        Client { ptr: ptr }
    }
}

mod generated {
    #![allow(dead_code,non_camel_case_types,unused_unsafe,unused_variables)]
    #![allow(non_upper_case_globals,non_snake_case,unused_imports)]

    pub mod interfaces {
        include!(concat!(env!("OUT_DIR"), "/wayland_interfaces.rs"));
    }

    pub mod server {
        // Imports that need to be available to submodules
        // but should not be in public API.
        // Will be fixable with pub(restricted).
        #[doc(hidden)] pub use {Resource, EventQueueHandle, Handler, Client};
        #[doc(hidden)] pub use super::interfaces;

        include!(concat!(env!("OUT_DIR"), "/wayland_api.rs"));
    }
}

pub mod sys {
    pub use wayland_sys::server::*;
    pub use wayland_sys::common::*;
}