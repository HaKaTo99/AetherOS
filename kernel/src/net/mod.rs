pub mod mesh;

pub struct DummyDevice;
impl DummyDevice {
    pub fn inject(&mut self, _data: alloc::vec::Vec<u8>) {}
}

pub struct NetworkStack {
    pub device: DummyDevice,
}

impl NetworkStack {
    pub fn new() -> Self { 
        Self { device: DummyDevice } 
    }
    pub fn poll(&mut self, _timestamp: i64) {}
}
