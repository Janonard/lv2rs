//! Wrappers for raw LV2 audio IO.
//!
//! The wrappers provided in this module increase the safety when dealing with the raw IO pointers
//! provided by a plugin's [`connect_port`](../trait.Plugin.html#tymythod.connect_port) function by
//! granting only safe access to the data.
//!
//! You should use these wrappers in your plugin struct, since they clearly communicate what type of
//! data they contain. If you only store raw pointers to the ports, you can not tell an
//! audio port from a parameter port only looking at the type, for example.

/// Wrapper for raw audio input lists.
pub struct AudioInputPort {
    raw: *const f32,
}

impl AudioInputPort {
    /// Create a new instance that points to null.
    pub fn new() -> Self {
        Self {
            raw: std::ptr::null(),
        }
    }

    /// Set the internal data pointer.
    ///
    /// This function should only be called by a plugin's `connect_port` function.
    pub fn connect(&mut self, raw: *const f32) {
        self.raw = raw
    }

    /// Try to create an immutable slice of the audio data with the given length.
    ///
    /// This function is unsafe since invalid slices can be created by passing an invalid sample
    /// count. Therefore, only a plugin's `run` function should use this function and must pass
    /// the sample count it received from the host.
    pub unsafe fn as_slice(&self, n_samples: u32) -> Option<&[f32]> {
        if self.raw.is_null() {
            None
        } else {
            Some(std::slice::from_raw_parts(self.raw, n_samples as usize))
        }
    }
}

/// Wrapper for raw audio output lists.
pub struct AudioOutputPort {
    raw: *mut f32,
}

impl AudioOutputPort {
    /// Create a new instance that points to null.
    pub fn new() -> Self {
        Self {
            raw: std::ptr::null_mut(),
        }
    }

    /// Set the internal data pointer.
    ///
    /// This function should only be called by a plugin's `connect_port` function.
    pub fn connect(&mut self, raw: *mut f32) {
        self.raw = raw;
    }

    /// Try to create a mutable slice of the audio data with the given length.
    ///
    /// This function is unsafe since invalid slices can be created by passing an invalid sample
    /// count. Therefore, only a plugin's `run` function should use this function and must pass
    /// the sample count it receives from the host.
    pub unsafe fn as_slice(&mut self, n_samples: u32) -> Option<&mut [f32]> {
        if self.raw.is_null() {
            None
        } else {
            Some(std::slice::from_raw_parts_mut(self.raw, n_samples as usize))
        }
    }
}

/// Wrapper for raw parameter inputs.
pub struct ParameterInputPort {
    raw: *const f32,
}

impl ParameterInputPort {
    /// Create a new instance that points to null.
    pub fn new() -> Self {
        Self {
            raw: std::ptr::null(),
        }
    }

    /// Set the internal data pointer.
    ///
    /// This function should only be called by a plugin's `connect_port` function.
    pub fn connect(&mut self, raw: *const f32) {
        self.raw = raw;
    }

    /// Try to access the parameter.
    ///
    /// This is just a wrapper for `self.raw.as_ref()`
    pub unsafe fn get(&self) -> Option<&f32> {
        self.raw.as_ref()
    }
}

/// Safer wrapper for raw parameter outputs.
pub struct ParameterOutputPort {
    raw: *mut f32,
}

impl ParameterOutputPort {
    /// Create a new instance that points to null.
    pub fn new() -> Self {
        Self {
            raw: std::ptr::null_mut(),
        }
    }

    /// Set the internal data pointer.
    ///
    /// This function should only be called by a plugin's `connect_port` function.
    pub fn connect(&mut self, raw: *mut f32) {
        self.raw = raw;
    }

    /// Try to access the parameter.
    ///
    /// This is just a wrapper for `self.raw.as_mut()`
    pub unsafe fn get_mut(&mut self) -> Option<&mut f32> {
        self.raw.as_mut()
    }
}
