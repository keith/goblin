macro_rules! io_error {
    ($fmt:expr, $($arg:tt)*) =>
        (
            Err(io::Error::new(io::ErrorKind::Other, format!($fmt, $($arg)*)))
        );
}
