use std::error::Error;

use bincode::{Encode, Decode};

use crate::interop;

use super::property::UnityProperty;

/// Represents a C# Class
#[derive(Debug, Copy, Encode, Decode)]
pub struct UnityClass {
    /// The inner pointer to the Class
    pub inner: i32,
}

unsafe impl Send for UnityClass {}
unsafe impl Sync for UnityClass {}

impl Clone for UnityClass {
    fn clone(&self) -> UnityClass {
        UnityClass { ..*self }
    }
}

impl UnityClass {
    pub fn get_name(&self) -> Result<String, Box<dyn Error>> {
        interop::imports::get_class_name(self)
    }

    pub fn get_property(&self, name: &str) -> Result<UnityProperty, Box<dyn Error>> {
        interop::imports::get_property(self, name)
    }
}