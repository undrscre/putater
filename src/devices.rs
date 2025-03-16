pub trait Device {
    fn read(&self, offset: u8) -> u8;
    fn write(&mut self, offset: u8, value: u8);
}

pub struct NullDevice;
impl Device for NullDevice {
    fn read(&self, _: u8) -> u8 { 0 }
    fn write(&mut self, offset: u8, value: u8) {
        println!("Screen received write at {offset}: {value}");
    }
}

pub struct ScreenDevice;
impl Device for ScreenDevice {
    fn read(&self, _: u8) -> u8 { 0 }
    fn write(&mut self, offset: u8, value: u8) {
        println!("Screen received write at {offset}: {value}");
    }
}

pub enum DeviceIDs {
    ScreenID = 1,
}