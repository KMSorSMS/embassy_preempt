use cortex_m_semihosting::debug;
#[cfg(feature = "panic-probe")]
use panic_probe as _;

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[cfg(feature = "defmt")]
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}
#[cfg(not(feature = "defmt"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

/// Terminates the application and makes a semihosting-capable debug tool exit
/// with status code 0.
pub fn exit() -> ! {
    loop {
        debug::exit(debug::EXIT_SUCCESS);
    }
}

/// Hardfault handler.
///
/// Terminates the application and makes a semihosting-capable debug tool exit
/// with an error. This seems better than the default, which is to spin in a
/// loop.
#[cortex_m_rt::exception]
unsafe fn HardFault(_frame: &cortex_m_rt::ExceptionFrame) -> ! {
    loop {
        debug::exit(debug::EXIT_FAILURE);
    }
}
