pub use lv2_raw::core::LV2Feature as Feature;

pub struct FeatureIterator {
    raw: *const *const crate::raw::LV2Feature,
}

impl FeatureIterator {
    pub fn new(raw: *const *const crate::raw::LV2Feature) -> Self {
        Self { raw: raw }
    }
}

impl std::iter::Iterator for FeatureIterator {
    type Item = &'static crate::raw::LV2Feature;

    fn next(&mut self) -> Option<Self::Item> {
        if self.raw.is_null() {
            None
        } else {
            let feature = unsafe { (*self.raw).as_ref() }.unwrap();
            self.raw = unsafe { self.raw.add(1) };
            Some(feature)
        }
    }
}