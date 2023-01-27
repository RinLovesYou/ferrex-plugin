use std::error::Error;

use bincode::{Encode, Decode};

use crate::interop::imports;

use super::{object::UnityObject, common::ArbitraryData};

/// Represents a C# Property
#[derive(Debug, Copy, Encode, Decode)]
pub struct UnityProperty {
    /// The inner pointer to the Property
    pub inner: i32,
}

unsafe impl Send for UnityProperty {}
unsafe impl Sync for UnityProperty {}

impl Clone for UnityProperty {
    fn clone(&self) -> UnityProperty {
        UnityProperty { ..*self }
    }
}

impl UnityProperty {
    pub fn get_name(&self) -> Result<String, Box<dyn Error>> {
        imports::get_property_name(self)
    }

    pub fn get_value(&self) -> Result<UnityObject, Box<dyn Error>> {
        imports::get_property_value(self, None)
    }

    pub fn set_value(&self, obj: Option<&UnityObject>, value: &ArbitraryData) -> Result<(), Box<dyn Error>> {
        imports::set_property_value(self, obj, value);
        Ok(())
    }
}