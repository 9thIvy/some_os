//Comments and code are all from phil-opp's blog_os
//At some point I will add backspace support and a shell

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};

mod serial;
mod vga_buffer;

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use blog_os::memory;
    use blog_os::memory::BootInfoFrameAllocater;
    use x86_64::VirtAddr;
    blog_os::init();
    println!("blog_os::init...[Ok]");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    //Will be used later
    #[allow(unused)]
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    #[allow(unused)]
    let mut frame_allocator = unsafe { BootInfoFrameAllocater::init(&boot_info.memory_map) };

    #[cfg(test)]
    test_main();

    blog_os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("`{}`", info);
    blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}
