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
    println!("taos v0.1");

    // Now see if we can read stuff:
    // Usually we can use #[test] modules in Rust, but it would convolute the
    // task at hand. So, we'll just add testing snippets.
    loop {
        if let Some(byte) = uart.get() {
            gpio.out_high(19);
            //use numtoa::NumToA;
            //let mut buffer = [0u8; 3];
            //println!("{}", (byte as u8).numtoa_str(10, &mut buffer));
            match byte {
                8 | 127 => {
                    // This is a backspace, so we essentially have
                    // to write a space and backup again:
                    //print!("{}{}{}", 8 as char, ' ', 8 as char);
                    print!("\x08 \x08");
                }
                10 | 13 => {
                    // Newline or carriage-return
                    println!();
                }
                0x1b => {
                    // Those familiar with ANSI escape sequences
                    // knows that this is one of them. The next
                    // thing we should get is the left bracket [
                    // These are multi-byte sequences, so we can take
                    // a chance and get from UART ourselves.
                    // Later, we'll button this up.
                    if let Some(bracket) = uart.get() {
                        if bracket == 91 {
                            // This is a right bracket! We're on our way!
                            if let Some(esc_cmd) = uart.get() {
                                match esc_cmd as char {
                                    'A' => {
                                        println!("That's the up arrow!");
                                    }
                                    'B' => {
                                        println!("That's the down arrow!");
                                    }
                                    'C' => {
                                        println!("That's the right arrow!");
                                    }
                                    'D' => {
                                        println!("That's the left arrow!");
                                    }
                                    _ => {
                                        println!("That's something else.....");
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {
                    //print!("{}", byte as char);
                    print!("x");
                }
            }
            gpio.out_low(19);
        }
    }
}

// ///////////////////////////////////
// / RUST MODULES
// ///////////////////////////////////

pub mod gpio;
pub mod uart;
