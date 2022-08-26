use core::panic::PanicInfo;

#[panic_handler]
extern "C" fn begin_unwind(_info: &PanicInfo) -> ! {
    loop {}
}

/// This function is the entry point for the unwinding process.
#[lang = "eh_personality"]
#[no_mangle]
extern "C" fn rust_eh_personality() -> ! {
    loop {}
}
