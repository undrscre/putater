use std::io::{empty, Empty, Read, Result, Write};

pub trait Device: Read + Write {}

pub struct NullDevice(Empty);

impl NullDevice {
    pub fn new() -> Self {
        Self(empty())
    }
}

impl Device for NullDevice {}

impl Read for NullDevice {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.0.read(buf)
    }
}

impl Write for NullDevice {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.0.flush()
    }
}

pub struct TestDevice { inner: Empty, initial_write: bool }

impl TestDevice {
    pub fn new() -> Self {
        Self { inner: empty(), initial_write: true }
    }
}

impl Device for TestDevice {}

impl Read for TestDevice {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if self.initial_write && buf.len() > 0 {
            println!("hello from testdevice! {:?}", buf);
            buf[0] = 128;
            self.initial_write = false;
        }

        self.inner.read(buf)
    }
}

impl Write for TestDevice {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.inner.write(buf)
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}
