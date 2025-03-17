pub trait Device {
    fn read(&self, offset: u8) -> u8;
    fn write(&mut self, offset: u8, value: u8);
}

pub struct NullDevice;
impl Device for NullDevice {
    fn read(&self, _: u8) -> u8 { 0 }
    fn write(&mut self, _: u8, _: u8) {}
}

pub struct TestDevice;
impl Device for TestDevice {
    fn read(&self, offset: u8) -> u8 { 
        match offset {
            0 => {
                println!("hello from testdevice! {0}", offset);
                128
            }
            _ => {0}
        }
    }
    fn write(&mut self, _offset: u8, _value: u8) {}
}