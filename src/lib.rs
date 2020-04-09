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
    let mut counter = 0 as u16;
    counter -= 1;
    while counter != 0 {
        counter -= 1;
    }
}

#[no_mangle]
extern "C" fn kmain() {

    let mut gpio = gpio::Gpio::new();
    gpio.init();
    gpio.out_high(19);
    gpio.out_high(20);
    gpio.out_high(21);
    gpio.out_high(22);

    let mut uart = uart::Uart::new(0x1001_3000);
    uart.init();

    loop {
        gpio.out_high(19);
        wait();
        gpio.out_low(19);
        wait();
    }

    loop {
        if let Some(byte) = uart.get() {
            //use numtoa::NumToA;
            //let mut buffer = [0u8; 3];
            //println!("{}", (byte as u8).numtoa_str(10, &mut buffer));
            match byte {
                8 | 127 => {
                    // This is a backspace, so we essentially have
                    // to write a space and backup again:
                    print!("{}{}{}", 8 as char, ' ', 8 as char);
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
                    print!("{}", byte as char);
                }
            }
        }
    }
}

// ///////////////////////////////////
// / RUST MODULES
// ///////////////////////////////////

pub mod gpio;
pub mod uart;
