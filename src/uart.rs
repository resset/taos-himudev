// uart.rs
// UART routines and driver

// use core::convert::TryInto;
use core::fmt::Error;
use core::fmt::Write;

pub struct Uart {
    base_address: usize,
}

impl Write for Uart {
    fn write_str(&mut self, out: &str) -> Result<(), Error> {
        for c in out.bytes() {
            self.put(c);
        }
        Ok(())
    }
}

impl Uart {
    pub fn new(base_address: usize) -> Self {
        Uart { base_address }
    }

    pub fn init(&mut self) {
        let ptr = self.base_address as *mut u32;
        unsafe {
            // Set divisor of 64 MHz clock (+1).
            let divisor: u32 = 555;
            ptr.add(6).write_volatile(divisor);

            // Set txen bit to 0.
            ptr.add(2).write_volatile(0);

            // Set rxen bit to 1 to enable receive.
            let rxctrl = ptr.add(3).read_volatile();
            ptr.add(3).write_volatile(rxctrl | 1 << 0);

            // Enable IOF for GPIO 16 (RX) and 17 (TX).
            // This is poorly documented in the manual as of now (04.2020).
            let ptr_gpio = 0x1001_2000 as *mut u32;
            let ioef_sel = ptr_gpio.add(15).read_volatile();
            // Select IOF0 for pins 16 and 17.
            ptr_gpio
                .add(15)
                .write_volatile(ioef_sel & !((1 << 16) | (1 << 17)));
            let ioef_en = ptr_gpio.add(14).read_volatile();
            // Enable IOF on pins 16 and 17 (those are no longer simple GPIOs).
            ptr_gpio
                .add(14)
                .write_volatile(ioef_en | (1 << 16) | (1 << 17));
        }
    }

    pub fn put(&mut self, c: u8) {
        let ptr = self.base_address as *mut u32;
        const TXDATA_FULL: u32 = 1 << 31;
        unsafe {
            // Wait until the FIFO full flag is cleared.
            while ptr.add(0).read_volatile() & TXDATA_FULL != 0 {}
            // Put character into a FIFO.
            ptr.add(0).write_volatile(c.into());
            // Set txen bit to 1 to enable transmitter.
            let txctrl = ptr.add(2).read_volatile();
            ptr.add(2).write_volatile(txctrl | 1 << 0);
        }
    }

    pub fn get(&mut self) -> Option<u8> {
        let ptr = self.base_address as *mut u32;
        const RXDATA_EMPTY: u32 = 1 << 31;
        unsafe {
            let rxdata = ptr.add(1).read_volatile();
            if rxdata & RXDATA_EMPTY != 0 {
                // The empty bit is 1, meaning no data
                None
            } else {
                // The empty bit is 0, we have data!
                Some(ptr.add(1).read_volatile() as u8)
            }
        }
    }
}
