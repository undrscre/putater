use crate::devices;
use crate::devices::Device;

pub struct MemoryBus {
    ram: [[u8; 256]; 251], // 250x256 = 64000 usable bytes of memory
    devices: [Option<Box<dyn Device>>; 5],
}

impl MemoryBus {
    pub fn new() -> Self {
        Self {
            ram: [[0; 256]; 251],
            devices: [None, None, None, None, None] // start out with none loaded
        }
    }

    pub fn attach_device(&mut self, device: Box<dyn Device>) -> Result<(), ()> {
        if let Some(slot) = self.devices.iter_mut().position(|d| d.is_none()) {
            self.devices[slot] = Some(device);
			println!("attached device!");
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn read(&mut self, page: u8, offset: u8) -> u8 {
        match page {
            251..=255 => {
                let idx = (page - 250) as usize;
                if let Some(device) = &self.devices[idx] {
                    device.read(offset)
                } else {
                    0
                }
            },
            _ => self.ram[page as usize][offset as usize],
        }
    }

    pub fn write(&mut self, page: u8, offset: u8, value: u8) {
        match page {
            250 => {
                match offset {
                    0 => {
                        let device: Box<dyn Device> = match value {
                            1 => Box::new(devices::ScreenDevice),
                            _ => Box::new(devices::NullDevice),
                        };
						self.attach_device(device).unwrap();
                    },
                    _ => {}
                }
            },
            251..=255 => {
                let idx = (page - 250) as usize;
                if let Some(device) = &mut self.devices[idx] {
                    device.write(offset, page)
                }
            },
            _ => self.ram[page as usize][offset as usize] = value
        }
    }
}