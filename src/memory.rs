use std::{fmt, io};

use crate::devices;
use crate::devices::Device;

pub enum MemoryBusError {
    NoDeviceOnPage(u8),
}

impl fmt::Display for MemoryBusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MemoryBusError::NoDeviceOnPage(page) => write!(f, "no device loaded on page {}", page),
        }
    }
}

pub struct MemoryBus {
    pub page: u8,
    ram: [io::Cursor<[u8; 256]>; 251], // 250x256 = 64000 usable bytes of memory
    devices: [Option<Box<dyn Device>>; 5],
}

impl MemoryBus {
    pub fn new() -> Self {
        Self {
            page: 0,
            ram: [const { io::Cursor::new([0; 256]) }; 251],
            devices: [None, None, None, None, None], // start out with none loaded
        }
    }

    pub fn attach_device(&mut self, device: Box<dyn Device>) -> Option<()> {
        let _ = self.devices
            .iter_mut()
            .find(|d| d.is_none())?
            .insert(device);
        Some(())
    }
}

impl io::Read for MemoryBus {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self.page {
            // device pages
            251..=255 => {
                let idx = (self.page - 251) as usize;
                if let Some(ref mut device) = &mut self.devices[idx] {
                    device.read(buf)
                } else {
                    Err(io::ErrorKind::NotFound.into())
                }
            }
            _ => self.ram[self.page as usize].read(buf),
        }
    }
}

impl io::Seek for MemoryBus {
    fn seek(&mut self, pos: io::SeekFrom) -> io::Result<u64> {
        if self.page > 250 {
            return Err(io::ErrorKind::NotSeekable.into())
        }

        self.ram[self.page as usize].seek(pos)
    }
}

impl io::Write for MemoryBus {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self.page {
            250 => {
                // @TODO offload device initialization somewhere else, it doesnt make sense in the memory bus
                Ok(buf.into_iter().try_fold(0, |acc, id| {
                    let device: Box<dyn Device> = match id {
                        0 => Box::new(devices::TestDevice::new()),
                        1 => Box::new(devices::graphics::DisplayDevice::new()),
                        _ => return Err(io::ErrorKind::NotFound.into()),
                    };

                    self.attach_device(device)
                        .ok_or(io::ErrorKind::QuotaExceeded)
                        .map(|_| acc + 1)
                })?)
            }
            251..=255 => {
                let idx = (self.page - 251) as usize;
                if let Some(device) = &mut self.devices[idx] {
                    device.write(buf)
                } else {
                    Err(io::ErrorKind::NotFound.into())
                }
            }
            _ => self.ram[self.page as usize].write(buf),
        }
    }

    // MemoryBus does not impl any buffering
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
