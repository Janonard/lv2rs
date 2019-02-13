pub struct AudioInputPort {
    raw: Option<*const f32>,
}

impl AudioInputPort {
    pub fn new() -> Self {
        Self { raw: None }
    }

    pub fn connect(&mut self, raw: *mut ()) {
        self.raw = Some(raw as *const f32);
    }

    pub fn iter(&self, n_samples: u32) -> Option<std::slice::Iter<f32>> {
        match self.raw {
            Some(raw) => {
                Some(unsafe { std::slice::from_raw_parts(raw, n_samples as usize) }.iter())
            }
            None => None,
        }
    }
}

pub struct AudioOutputPort {
    raw: Option<*mut f32>,
}

impl AudioOutputPort {
    pub fn new() -> Self {
        Self { raw: None }
    }

    pub fn connect(&mut self, raw: *mut ()) {
        self.raw = Some(raw as *mut f32);
    }

    pub fn iter_mut(&mut self, n_samples: u32) -> Option<std::slice::IterMut<f32>> {
        match self.raw {
            Some(raw) => {
                Some(unsafe { std::slice::from_raw_parts_mut(raw, n_samples as usize) }.iter_mut())
            }
            None => None,
        }
    }
}

pub struct ParameterInputPort {
    raw: Option<*const f32>,
}

impl ParameterInputPort {
    pub fn new() -> Self {
        Self { raw: None }
    }

    pub fn connect(&mut self, raw: *mut ()) {
        self.raw = Some(raw as *const f32);
    }

    pub fn get(&self) -> Option<f32> {
        match self.raw {
            Some(raw) => Some(unsafe { *raw }),
            None => None,
        }
    }
}

pub struct ParameterOutputPort {
    raw: Option<*mut f32>,
}

impl ParameterOutputPort {
    pub fn new() -> Self {
        Self { raw: None }
    }

    pub fn connect(&mut self, raw: *mut ()) {
        self.raw = Some(raw as *mut f32);
    }

    pub fn get_mut(&mut self) -> Option<&mut f32> {
        match self.raw {
            Some(raw) => Some(unsafe { raw.as_mut() }.unwrap()),
            None => None,
        }
    }
}
