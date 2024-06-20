#![no_std]
#![no_main]

extern crate libc;

#[no_mangle]
extern "C" fn main() -> libc::c_int {
    panic!("I just panic every time");
}

#[panic_handler]
fn my_panic(panic_info: &core::panic::PanicInfo) -> ! {
    use arrayvec::ArrayString;
    use core::fmt::Write;

    let message = panic_info.message();
    let location = panic_info.location().unwrap();

    let mut debug_msg = ArrayString::<1024>::new();
    let _ = write!(&mut debug_msg, "panicked with '{message}' at '{location}'");
    let _ = debug_msg.try_push_str("\0");

    unsafe {
        libc::puts(debug_msg.as_ptr() as *const _);
        libc::exit(libc::EXIT_FAILURE)
    }
}
