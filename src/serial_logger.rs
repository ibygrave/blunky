use core::{cell::RefCell, fmt::Write, mem::MaybeUninit};
use nb::block;
use static_box::Box;
use void::ResultVoidExt;

/// Trait that serial uarts must implement to be usable in this logger,
/// and a default implementation.
pub trait SerialWriter: embedded_hal::serial::Write<u8, Error = void::Void> {}
impl<T> SerialWriter for T where T: embedded_hal::serial::Write<u8, Error = void::Void> {}

impl Write for dyn SerialWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // Output string as bytes
        for b in s.as_bytes() {
            block!(self.write(*b)).void_unwrap();
        }

        block!(self.flush()).void_unwrap();

        Ok(())
    }
}

/// log::Log implementation that writes to a registered serial uart.
struct SerialLogger {
    level: log::Level,
}

impl log::Log for SerialLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            let mut writer = unsafe { WRITER.assume_init_ref().borrow_mut() };
            writeln!(writer, "{}", record.args()).unwrap()
        }
    }

    fn flush(&self) {}
}

/// Global static memory to store the registered serial uart.
static mut MEM: [u8; 8] = [0_u8; 8];
static mut WRITER: MaybeUninit<RefCell<Box<dyn SerialWriter>>> = MaybeUninit::uninit();

/// The active logger.
static mut LOGGER: SerialLogger = SerialLogger {
    level: log::Level::Info,
};

/// Register a serial uart and active the logger.
/// SAFETY: This must be called before any logging macros.
pub fn init<T>(serial: T, level: log::Level) -> Result<(), log::SetLoggerError>
where
    T: SerialWriter + 'static,
{
    unsafe {
        LOGGER.level = level;
        WRITER = MaybeUninit::new(RefCell::new(Box::<'static, dyn SerialWriter>::new(
            &mut MEM, serial,
        )));
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        log::set_logger_racy(&LOGGER).map(|()| log::set_max_level_racy(level.to_level_filter()))
    }
}
