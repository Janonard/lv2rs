pub struct InputBuffer {
    raw: Option<*const f32>,
}

impl InputBuffer {
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

pub struct OutputBuffer {
    raw: Option<*mut f32>,
}

impl OutputBuffer {
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

pub struct InputParameter {
    raw: Option<*const f32>,
}

impl InputParameter {
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

pub struct OutputParameter {
    raw: Option<*mut f32>,
}

impl OutputParameter {
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
