use std::error::Error;

use bincode::{Encode, Decode};

use crate::interop::{imports::get_assembly_name, self};

use super::class::UnityClass;

/// Represents a C# Assembly
#[derive(Debug, Copy, Encode, Decode)]
pub struct UnityAssembly {
    /// The inner pointer to the Assembly
    pub inner: i32,
}

unsafe impl Send for UnityAssembly {}
unsafe impl Sync for UnityAssembly {}

impl Clone for UnityAssembly {
    fn clone(&self) -> UnityAssembly {
        UnityAssembly { ..*self }
    }
}

impl UnityAssembly {
    pub fn get_name(&self) -> Result<String, Box<dyn Error>> {
        get_assembly_name(self)
    }

    pub fn get_class(&self, namespace: &str, name: &str) -> Result<UnityClass, Box<dyn Error>> {
        interop::imports::get_class(self, namespace, name)
    }
}