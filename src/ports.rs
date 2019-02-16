pub struct AudioInputPort {
    raw: *const f32,
}

impl AudioInputPort {
    pub fn new() -> Self {
        Self {
            raw: std::ptr::null(),
        }
    }

    pub fn connect(&mut self, raw: *const f32) {
        self.raw = raw
    }

    pub unsafe fn as_slice(&self, n_samples: u32) -> Option<&[f32]> {
        if self.raw.is_null() {
            None
        } else {
            Some(std::slice::from_raw_parts(self.raw, n_samples as usize))
        }
    }
}

pub struct AudioOutputPort {
    raw: *mut f32,
}

impl AudioOutputPort {
    pub fn new() -> Self {
        Self {
            raw: std::ptr::null_mut(),
        }
    }

    pub fn connect(&mut self, raw: *mut f32) {
        self.raw = raw;
    }

    pub unsafe fn as_slice(&mut self, n_samples: u32) -> Option<&mut [f32]> {
        if self.raw.is_null() {
            None
        } else {
            Some(std::slice::from_raw_parts_mut(self.raw, n_samples as usize))
        }
    }
}

pub struct ParameterInputPort {
    raw: *const f32,
}

impl ParameterInputPort {
    pub fn new() -> Self {
        Self {
            raw: std::ptr::null(),
        }
    }

    pub fn connect(&mut self, raw: *const f32) {
        self.raw = raw;
    }

    pub unsafe fn get(&self) -> Option<&f32> {
        self.raw.as_ref()
    }
}

pub struct ParameterOutputPort {
    raw: *mut f32,
}

impl ParameterOutputPort {
    pub fn new() -> Self {
        Self {
            raw: std::ptr::null_mut(),
        }
    }

    pub fn connect(&mut self, raw: *mut f32) {
        self.raw = raw;
    }

    pub unsafe fn get_mut(&mut self) -> Option<&mut f32> {
        self.raw.as_mut()
    }
}
