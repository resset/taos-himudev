pub struct Gpio {
    base_address: usize,
}

impl Gpio {
    pub fn new() -> Self {
        let base_address = 0x1001_2000 as usize;
        Gpio { base_address }
    }

    unsafe fn _mmio_write(address: usize, offset: usize, value: u32) {
        let reg = address as *mut u32;
        reg.add(offset).write_volatile(value);
    }

    unsafe fn _mmio_read(address: usize, offset: usize) -> u32 {
        let reg = address as *mut u32;
        reg.add(offset).read_volatile()
    }

    pub fn init(&mut self) {
        let _ptr = self.base_address as *mut u32;
        unsafe {
            Gpio::_mmio_write(0x10012008, 0, 0x780000);
            Gpio::_mmio_write(0x1001200c, 0, 0x000000);
        }
    }

    pub fn out_high(&mut self, pin: u8) {
        let ptr = self.base_address as *mut u32;
        unsafe {
            let output_en = ptr.add(2).read_volatile();
            ptr.add(2).write_volatile(output_en | 1 << pin);
            let output_val = ptr.add(3).read_volatile();
            ptr.add(3).write_volatile(output_val | 1 << pin);
        }
    }

    pub fn out_low(&mut self, pin: u8) {
        let ptr = self.base_address as *mut u32;
        unsafe {
            let output_en = ptr.add(2).read_volatile();
            ptr.add(2).write_volatile(output_en | 1 << pin);
            let output_val = ptr.add(3).read_volatile();
            ptr.add(3).write_volatile(output_val & !(1 << pin));
        }
    }

    pub fn get(&mut self) -> Option<u8> {
        let ptr = self.base_address as *mut u32;
        unsafe {
            let rxdata = ptr.add(1).read_volatile() as u32;
            if rxdata & 0x8000_000 != 0 {
                // The empty bit is 1, meaning no data
                None
            } else {
                // The empty bit is 0, meaning data!
                Some(ptr.add(1).read_volatile() as u8)
            }
        }
    }
}
