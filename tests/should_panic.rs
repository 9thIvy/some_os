#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]

use blog_os::{exit_qemu, serial_print, serial_println, QemuExitCode};

//the panic handler returns a success because tests are expected to panic
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    serial_println!("[Ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    loop {}
}
