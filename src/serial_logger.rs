use core::cell::RefCell;
use core::mem::MaybeUninit;

pub static mut WRITER: MaybeUninit<RefCell<crate::uart::Uart>> = MaybeUninit::uninit();

/// Register a serial uart
/// SAFETY: This must be called before any logging macros.
pub fn init(uart: crate::uart::Uart) {
    unsafe {
        WRITER = MaybeUninit::new(RefCell::new(uart));
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    }
}
