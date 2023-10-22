#[cfg(profile = "release")]
#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => {{}};
}

#[cfg(not(profile = "release"))]
#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => (write!(unsafe { $crate::serial_logger::WRITER.assume_init_ref().borrow_mut() }, $($arg)+).unwrap())
}
