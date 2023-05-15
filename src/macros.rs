/// Shortcut for log::log!() + println!()
/// 
/// ### Example
/// 
/// ```
/// log!("trace", "Unable to connect to WS gateway: {}", e.to_string());
/// ```
#[macro_export]
macro_rules! log {
    ($lvl:expr, $msg:expr) => ({
        let lvl = match $lvl {
            "trace" => log::Level::Trace,
            "debug" => log::Level::Debug,
            "info" => log::Level::Info,
            "warn" => log::Level::Warn,
            "error" => log::Level::Error,
            _ => unreachable!(),
        };
        log::log!(lvl, $msg);
        println!($msg)
    });
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

/// Shortcut for log::log!() + println!(). Returns an Ok variant.
/// 
/// ### Example
/// 
/// ```
/// return log_ok!("trace", "Unable to connect to WS gateway: {}", e.to_string());
/// ```
#[macro_export]
macro_rules! log_ok {
    ($lvl:expr, $msg:expr) => ({
        $crate::log!($lvl, $msg);
        Ok(())
    });
    ($lvl:expr, $msg:expr, $($arg:tt)*) => ({
        $crate::log!($lvl, $msg, $($arg)*);
        Ok(())
    });
}

/// Shortcut for log::log!() + println!(). Returns an Err variant constructed using anyhow::anyhow!().
/// 
/// ### Example
/// 
/// ```
/// return log_err!("trace", "Unable to connect to WS gateway: {}", e.to_string());
/// ```
#[macro_export]
macro_rules! log_err {
    ($lvl:expr, $msg:expr) => ({
        $crate::log!($lvl, $msg);
        Err(anyhow::anyhow!($msg))
    });
    ($lvl:expr, $msg:expr, $($arg:tt)*) => ({
        $crate::log!($lvl, $msg, $($arg)*);
        Err(anyhow::anyhow!($msg, $($arg)*))
    });
}
