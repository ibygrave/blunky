#[cfg(profile = "release")]
#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => {{}};
}

#[cfg(not(profile = "release"))]
#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => (write!( &mut $crate::uart::Writer , $($arg)+).unwrap())
}
