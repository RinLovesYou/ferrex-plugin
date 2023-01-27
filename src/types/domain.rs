use std::error::Error;

use bincode::{Encode, Decode};

use crate::interop;

use super::assembly::UnityAssembly;

/// Represents a C# Domain
#[derive(Debug, Copy, Encode, Decode)]
pub struct UnityDomain {
    /// The inner pointer to the Domain
    pub inner: i32,
}

unsafe impl Send for UnityDomain {}
unsafe impl Sync for UnityDomain {}

impl Clone for UnityDomain {
    fn clone(&self) -> UnityDomain {
        UnityDomain { ..*self }
    }
}

impl UnityDomain {
    pub fn get_assemblies(&self) -> Result<Vec<UnityAssembly>, Box<dyn Error>> {
        interop::imports::get_assemblies()
    }

    pub fn get_assembly(&self, name: &str) -> Result<UnityAssembly, Box<dyn Error>> {
        interop::imports::get_assembly(&name.to_string())
    }
}