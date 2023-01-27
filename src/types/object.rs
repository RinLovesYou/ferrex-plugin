use bincode::{Encode, Decode};

/// Represents a C# Property
#[derive(Debug, Copy, Encode, Decode)]
pub struct UnityObject {
    /// The inner pointer to the Object
    pub inner: i32,
}

unsafe impl Send for UnityObject {}
unsafe impl Sync for UnityObject {}

impl Clone for UnityObject {
    fn clone(&self) -> UnityObject {
        UnityObject { ..*self }
    }
}

impl UnityObject {

}