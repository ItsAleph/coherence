#[macro_export]
macro_rules! log {
    ($lvl:expr, $msg:expr, $($arg:tt)*) => ({
        let lvl = match $lvl {
            "trace" => log::Level::Trace,
            "debug" => log::Level::Debug,
            "info" => log::Level::Info,
            "warn" => log::Level::Warn,
            "error" => log::Level::Error,
            _ => unreachable!(),
        };
        log::log!(lvl, $msg, $($arg)*);
        println!($msg, $($arg)*)
    });
}

/// Logs message, then returns [Ok](anyhow::Result::Ok).
/// log_ok("trace", "Unable to connect to WS gateway: {}", e.to_string());
#[macro_export]
macro_rules! log_ok {
    ($lvl:expr, $msg:expr, $($arg:tt)*) => ({
        $crate::log!($lvl, $msg, $($arg)*);
        Ok(())
    });
}

#[macro_export]
macro_rules! log_err {
    ($lvl:expr, $msg:expr, $($arg:tt)*) => ({
        $crate::log!($lvl, $msg, $($arg)*);
        Err(anyhow::anyhow!($msg, $($arg)*))
    });
}
