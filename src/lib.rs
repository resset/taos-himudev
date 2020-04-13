// Steve Operating System
// Stephen Marz
// 21 Sep 2019
#![no_std]
#![feature(panic_info_message, asm)]

// ///////////////////////////////////
// / RUST MACROS
// ///////////////////////////////////
#[macro_export]
macro_rules! print {
    ($($args:tt)+) => {{
        use core::fmt::Write;
        let _ = write!(crate::uart::Uart::new(0x1001_3000), $($args)+);
    }};
}
#[macro_export]
macro_rules! println
{
    () => ({
        print!("\r\n")
    });
    ($fmt:expr) => ({
        print!(concat!($fmt, "\r\n"))
    });
    ($fmt:expr, $($args:tt)+) => ({
        print!(concat!($fmt, "\r\n"), $($args)+)
    });
}

// ///////////////////////////////////
// / LANGUAGE STRUCTURES / FUNCTIONS
// ///////////////////////////////////
#[no_mangle]
extern "C" fn eh_personality() {}
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    print!("Aborting: ");
    if let Some(_p) = info.location() {
        println!(
            "line {}, file {}: {}",
            _p.line(),
            _p.file(),
            info.message().unwrap()
        );
    } else {
        println!("no information available.");
    }
    abort();
}
#[no_mangle]
extern "C" fn abort() -> ! {
    loop {
        unsafe {
            asm!("wfi"::::"volatile");
        }
    }
}

// ///////////////////////////////////
// / CONSTANTS
// ///////////////////////////////////

// ///////////////////////////////////
// / ENTRY POINT
// ///////////////////////////////////
#[no_mangle]
fn wait() {
    let mut counter = 65535 as u16;
    while counter != 0 {
        counter -= 1;
    }
}

#[no_mangle]
unsafe fn _mmio_write(address: usize, offset: usize, value: u32) {
    let reg = address as *mut u32;
    reg.add(offset).write_volatile(value);
}

#[no_mangle]
extern "C" fn kmain() {
    // Set pllsel=1, this effectively selects XTAL clock source.
    // PLL is not bypassed, so our hfclk and tlclk = 64 MHz.
    unsafe {
        _mmio_write(0x10008008, 0, 0x00030df1);
    }

    let mut gpio = gpio::Gpio::new();
    gpio.init();
    gpio.out_low(19);
    gpio.out_low(20);
    gpio.out_low(21);
    gpio.out_low(22);

    let mut uart = uart::Uart::new(0x1001_3000);
    uart.init();

    loop {
        gpio.out_high(20);
        wait();
        gpio.out_low(20);
        gpio.out_high(19);
        wait();
        gpio.out_low(19);
        gpio.out_high(21);
        wait();
        gpio.out_low(21);
        gpio.out_high(22);
        wait();
        gpio.out_low(22);
        println!("Hello, World!");
    }
}

// ///////////////////////////////////
// / RUST MODULES
// ///////////////////////////////////

pub mod gpio;
pub mod uart;
