extern crate lv2rs_core as lv2;
extern crate lv2rs_urid as urid;

use std::ffi::CStr;

struct UridTestPlugin {
    input: lv2::ports::AudioInputPort,
    output: lv2::ports::AudioOutputPort,
}

impl lv2::Plugin for UridTestPlugin {
    fn instantiate(
        _descriptor: &lv2::Descriptor,
        _rate: f64,
        _bundle_path: &CStr,
        features: Option<&lv2::FeaturesList>,
    ) -> Option<Self> {
        let features = features?;

        let map = urid::Map::try_from_features(features).unwrap();
        let unmap = urid::Unmap::try_from_features(features).unwrap();

        let map_uri = CStr::from_bytes_with_nul(urid::uris::MAP_URI)
            .unwrap()
            .to_owned();

        let map_urid = map.map(&map_uri);

        println!("{:?} is mapped to {}", map_uri, map_urid);

        let map_uri = unmap.unmap(map_urid);

        println!("{} is mapped to {:?}", map_urid, map_uri);

        Some(Self {
            input: lv2::ports::AudioInputPort::new(),
            output: lv2::ports::AudioOutputPort::new(),
        })
    }

    unsafe fn connect_port(&mut self, port: u32, data: *mut ()) {
        match port {
            0 => self.input.connect(data as *const f32),
            1 => self.output.connect(data as *mut f32),
            _ => (),
        }
    }

    fn run(&mut self, n_samples: u32) {
        let input = unsafe { self.input.as_slice(n_samples) }.unwrap();
        let output = unsafe { self.output.as_slice(n_samples) }.unwrap();
        output.copy_from_slice(input);
    }
}

lv2::lv2_main!(
    lv2,
    UridTestPlugin,
    b"https://github.com/Janonard/UridTest\0"
);
