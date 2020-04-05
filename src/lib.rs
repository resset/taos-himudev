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

unsafe fn mmio_write(address: usize, offset: usize, value: u32) {
	let reg = address as *mut u32;
	reg.add(offset).write_volatile(value);
}

unsafe fn mmio_read(address: usize, offset: usize) -> u32 {
	let reg = address as *mut u32;
	reg.add(offset).read_volatile()
}

// ///////////////////////////////////
// / CONSTANTS
// ///////////////////////////////////

// ///////////////////////////////////
// / ENTRY POINT
// ///////////////////////////////////
#[no_mangle]
extern "C" fn kmain() {
    unsafe {
        mmio_write(0x10012008, 0, 0x780000);
        mmio_write(0x1001200c, 0, 0x780000);
    }
    loop {
    }
}

// ///////////////////////////////////
// / RUST MODULES
// ///////////////////////////////////
